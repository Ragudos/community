onmessage = (e) => {
    console.log("Worker: Received message. Verifying captcha...");
    var token = e.data;
    var api_endpoint = "/recaptcha/verify";
    var xhr = new XMLHttpRequest();

    xhr.open("POST", api_endpoint, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onreadystatechange = () => {
        postMessage({
            status: xhr.status,
            responseData: xhr.responseText
        });
    }

    xhr.send(JSON.stringify({ token: token }));
}
