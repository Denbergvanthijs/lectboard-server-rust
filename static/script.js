const TIMEOUT = 200;

var degree = 180;
var cache = 0;

window.onload = function updateImage() {
    document.getElementById("whiteboard").src = `/static/result.png?cache=${cache++}`;
    setTimeout(updateImage, TIMEOUT);
}

function rot180() {
    document.getElementById("whiteboard").style.transform = `rotate(${degree}deg)`;
    degree += 180;
}

const userAction = async () => {
    const response = await fetch("/hostname", {
        headers: {
            "Content-Type": "application/json"
        }
    });
    const json = await response.json();
    window.alert(`Your hostname is: ${json.hostname}\nYour port is: ${json.port}`);
}