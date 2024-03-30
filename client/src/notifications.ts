export function initNotifications() {
    const metaLoggedinIndicator = document.querySelector(
        "meta[name='is_logged_in']",
    );
    const isLoggedin =
        metaLoggedinIndicator &&
        metaLoggedinIndicator.getAttribute("content") === "true";

    maybeStartNotifications(isLoggedin);
}

const NOTIFICATIONS_STATE = {
    isConnected: false,
};

function maybeStartNotifications(isLoggedin: boolean) {
    if (!isLoggedin) {
        return;
    }

    let retryTime = 1;
    const uri = "/notifications/sse";

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
            "color: lightblue; font-size: 2px;",
        );
        setTimeout(connection, (() => timeout * 1000)());
    };
}

function onOpen(uri: string, resetRetryTime: () => void) {
    return () => {
        setConnectionStatus(true);
        console.log(
            `%cConnected to SSE at ${uri}`,
            "color: lightblue; font-size: 2px;",
        );
        resetRetryTime();
    };
}

function onMessage() {
    return (evt: CustomEvent) => {
        console.log(evt);
    };
}

function setConnectionStatus(status: boolean) {
    NOTIFICATIONS_STATE.isConnected = status;

    const statusIndicator = document.getElementById(
        "main-notifications .status",
    );

    statusIndicator.classList.remove("disconnected", "connected");
    statusIndicator.classList.add(status ? "connected" : "disconnected");
}
