
function get_greeting() {
    let now = new Date();
    let hour = now.getHours();
    let greeting;

    if (hour >= 22) {
        greeting = "Good Night";
    } else if (hour >= 18) {
        greeting = "Good Evening";
    } else if (hour >= 12) {
        greeting = "Good Afternoon";
    } else {
        greeting = "Good Morning";
    }

    return greeting;
}

function set_greeting_text() {
    document.getElementById("greeting_span").textContent = get_greeting();
}
