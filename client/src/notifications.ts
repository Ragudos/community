export function initNotifications() {
    window.startReceivingNotifications = function () {
        // We get all because of a bug with head-support
        const metaLoggedinIndicators = document.querySelectorAll(
            "meta[name='is_logged_in']",
        );
        const metaLoggedinIndicator =
            metaLoggedinIndicators[metaLoggedinIndicators.length - 1];
        const isLoggedin =
            metaLoggedinIndicator &&
            metaLoggedinIndicator.getAttribute("content") === "true";

        maybeStartNotifications(isLoggedin);
    };
}

const NOTIFICATIONS_STATE = {
    isConnected: false,
};

function maybeStartNotifications(isLoggedin: boolean) {
    if (!isLoggedin) {
        return;
    }

    let retryTime = 1;
    const uri = "/notifications/api/sse";

    function resetRetryTime() {
        retryTime = 1;
    }

    function changeRetryTime(newTime: number) {
        retryTime = newTime;
    }

    function connection() {
        const sse = new EventSource(uri);

        sse.addEventListener("message", onMessage());
        sse.addEventListener("open", onOpen(uri, resetRetryTime));
        sse.addEventListener(
            "error",
            onError(uri, retryTime, changeRetryTime, sse, connection),
        );
    }

    connection();
}

function onError(
    uri: string,
    retryTime: number,
    changeRetryTime: (newTime: number) => void,
    sse: EventSource,
    connection: () => void,
) {
    return () => {
        setConnectionStatus(false);
        sse.close();

        let timeout = retryTime;

        changeRetryTime(Math.min(64, retryTime * 2));

        console.log(
            `%cDisconnected from SSE at ${uri}. Reconnecting in ${timeout} seconds`,
            "color: lightblue; font-size: 12px;",
        );
        setTimeout(connection, (() => timeout * 1000)());
    };
}

function onOpen(uri: string, resetRetryTime: () => void) {
    return () => {
        setConnectionStatus(true);
        console.log(
            `%cConnected to SSE at ${uri}`,
            "color: lightblue; font-size: 12px;",
        );
        resetRetryTime();
    };
}

type NotificationData = {
    _id: number;
    _recipient_id: number;
    _sender_id: number;
    message: string;
    sent_at: string;
    link: string;
};

function parseNotificationData(data: unknown): data is NotificationData {
    if (typeof data !== "object") {
        return false;
    }

    if (
        !("_id" in data) ||
        !("_recipient_id" in data) ||
        !("_sender_id" in data) ||
        !("message" in data) ||
        !("sent_at" in data) ||
        !("link" in data)
    ) {
        return false;
    }

    return true;
}

// TODO: Simplify this. Too verbose
function onMessage() {
    return (evt: MessageEvent) => {
        const data = JSON.parse(evt.data);

        if (!parseNotificationData(data)) {
            throw new Error("Invalid notification data");
        }

        const notificationsContainer = document.getElementById("notifications");

        const notification = document.createElement("li");
        notification.id = `notification-${data._id}`;
        notification.setAttribute("hx-boost", "false");
        notification.setAttribute("data-type", "unread-notification");

        const wrapper = document.createElement("div");
        notification.appendChild(wrapper);

        const readIndicator = document.createElement("div");
        readIndicator.classList.add("read-indicator");
        readIndicator.id = `notification-read-indicator-${data._id}`;
        wrapper.appendChild(readIndicator);

        const profilePicture = document.createElement("div");
        profilePicture.id = `profile${data._sender_id}-${data._id}`;
        profilePicture.style.width = "40px";
        profilePicture.style.height = "40px";
        profilePicture.style.flexShrink = "0";

        const profilePictureChild = document.createElement("div");
        profilePictureChild.setAttribute(
            "hx-target",
            `#profile${data._sender_id}-${data._id}`,
        );
        profilePictureChild.setAttribute(
            "hx-get",
            `/user/api/${data._sender_id}/img`,
        );
        profilePictureChild.setAttribute("hx-trigger", "load once");

        profilePicture.appendChild(profilePictureChild);
        wrapper.appendChild(profilePicture);

        const notificationsMetadata = document.createElement("div");
        notificationsMetadata.classList.add("notifications-metadata");

        const message = document.createElement("div");
        message.classList.add("message");
        message.textContent = data.message;
        notificationsMetadata.appendChild(message);

        const contentFooter = document.createElement("div");
        contentFooter.classList.add("notification-content-footer");

        const time = document.createElement("time");
        time.textContent = "Just now";

        contentFooter.appendChild(time);

        const contentActions = document.createElement("div");
        contentActions.classList.add("notification-content-actions");

        const deleteNotificationForm = document.createElement("form");
        deleteNotificationForm.setAttribute(
            "hx-delete",
            "/notifications/api/delete",
        );
        deleteNotificationForm.setAttribute("hx-target-error", "#nothing");
        deleteNotificationForm.setAttribute("hx-swap", "outerHTML");
        deleteNotificationForm.setAttribute(
            "hx-target",
            `#notification-${data._id}`,
        );

        const deleteInput = document.createElement("input");
        deleteInput.type = "hidden";
        deleteInput.name = "notification_id";
        deleteInput.value = `${data._id}`;

        const deleteSubmitBtn = document.createElement("button");
        deleteSubmitBtn.type = "submit";
        deleteSubmitBtn.classList.add("square");
        deleteSubmitBtn.setAttribute("aria-label", "Delete this notifications");
        deleteSubmitBtn.setAttribute("title", "Delete this notification");
        deleteSubmitBtn.innerHTML = `
            <svg xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20" fill="currentColor"
                class="w-5 h-5">
                <path fill-rule="evenodd"
                    d="M8.75 1A2.75 2.75 0 0 0 6 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 1 0 .23 1.482l.149-.022.841 10.518A2.75 2.75 0 0 0 7.596 19h4.807a2.75 2.75 0 0 0 2.742-2.53l.841-10.52.149.023a.75.75 0 0 0 .23-1.482A41.03 41.03 0 0 0 14 4.193V3.75A2.75 2.75 0 0 0 11.25 1h-2.5ZM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4ZM8.58 7.72a.75.75 0 0 0-1.5.06l.3 7.5a.75.75 0 1 0 1.5-.06l-.3-7.5Zm4.34.06a.75.75 0 1 0-1.5-.06l-.3 7.5a.75.75 0 1 0 1.5.06l.3-7.5Z"
                    clip-rule="evenodd" />
            </svg>
        `;

        deleteNotificationForm.appendChild(deleteInput);
        deleteNotificationForm.appendChild(deleteSubmitBtn);

        contentActions.appendChild(deleteNotificationForm);

        if (data.link) {
            const markAsReadForm = document.createElement("form");
            markAsReadForm.setAttribute("hx-patch", "/notifications/api/read");
            markAsReadForm.setAttribute("hx-target-error", "#nothing");
            markAsReadForm.setAttribute("hx-swap", "outerHTML");
            markAsReadForm.setAttribute(
                "hx-indicator",
                "#notifications-loader",
            );
            markAsReadForm.classList.add("mark-as-read");

            const markAsReadInput = document.createElement("input");
            markAsReadInput.type = "hidden";
            markAsReadInput.name = "notification_id";
            markAsReadInput.value = `${data._id}`;

            markAsReadForm.appendChild(markAsReadInput);

            const markAsReadSubmitBtn = document.createElement("button");
            markAsReadSubmitBtn.type = "submit";
            markAsReadSubmitBtn.classList.add("square");
            markAsReadSubmitBtn.setAttribute(
                "aria-label",
                "Read morea and mark as read",
            );
            markAsReadSubmitBtn.setAttribute(
                "title",
                "Read more and mark as read",
            );
            markAsReadSubmitBtn.innerHTML = `
                <svg xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20" fill="currentColor"
                    class="w-5 h-5">
                    <path
                        d="M10 12.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
                    <path fill-rule="evenodd"
                        d="M.664 10.59a1.651 1.651 0 0 1 0-1.186A10.004 10.004 0 0 1 10 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0 1 10 17c-4.257 0-7.893-2.66-9.336-6.41ZM14 10a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
                        clip-rule="evenodd" />
                </svg>
            `;

            markAsReadForm.appendChild(markAsReadSubmitBtn);
            contentActions.appendChild(markAsReadForm);
        } else {
            const anchorTag = document.createElement("a");
            anchorTag.classList.add("square");
            anchorTag.setAttribute("href", data.link);
            anchorTag.setAttribute(
                "aria-label",
                "Read more about this notification",
            );
            anchorTag.setAttribute(
                "title",
                "Read more about this notification",
            );
            anchorTag.innerHTML = `
                <svg xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20" fill="currentColor"
                    class="w-5 h-5">
                    <path
                        d="M10 12.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
                    <path fill-rule="evenodd"
                        d="M.664 10.59a1.651 1.651 0 0 1 0-1.186A10.004 10.004 0 0 1 10 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0 1 10 17c-4.257 0-7.893-2.66-9.336-6.41ZM14 10a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
                        clip-rule="evenodd" />
                </svg>
            `;

            contentActions.appendChild(anchorTag);
        }

        contentFooter.appendChild(contentActions);
        notificationsMetadata.appendChild(contentFooter);
        wrapper.appendChild(notificationsMetadata);
        notification.appendChild(wrapper);
        notificationsContainer.prepend(notification);

        window.htmx.process(notification);

        const countIndicator = document.getElementById(
            "unread-notifications-count",
        );
        const currCount = countIndicator.textContent;

        if (currCount && !isNaN(parseInt(currCount))) {
            countIndicator.textContent = (parseInt(currCount) + 1).toString();
        } else {
            countIndicator.textContent = "1";
        }
    };
}

function setConnectionStatus(status: boolean) {
    NOTIFICATIONS_STATE.isConnected = status;

    const statusIndicator = document.querySelector(
        "#main-notifications .status",
    );

    statusIndicator.classList.remove("disconnected", "connected");
    statusIndicator.classList.add(status ? "connected" : "disconnected");
    statusIndicator.innerHTML = status
        ? `
            synced
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4">
                <path fill-rule="evenodd" d="M12.416 3.376a.75.75 0 0 1 .208 1.04l-5 7.5a.75.75 0 0 1-1.154.114l-3-3a.75.75 0 0 1 1.06-1.06l2.353 2.353 4.493-6.74a.75.75 0 0 1 1.04-.207Z" clip-rule="evenodd" />
            </svg>
        `
        : `
            unsynced
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4">
                <path d="M5.28 4.22a.75.75 0 0 0-1.06 1.06L6.94 8l-2.72 2.72a.75.75 0 1 0 1.06 1.06L8 9.06l2.72 2.72a.75.75 0 1 0 1.06-1.06L9.06 8l2.72-2.72a.75.75 0 0 0-1.06-1.06L8 6.94 5.28 4.22Z" />
            </svg>
        `;
}
