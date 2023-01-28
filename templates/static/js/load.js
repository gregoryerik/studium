
function get_greeting() {
    let now = new Date();
    let hour = now.getHours();
    let greeting;
    let additional;

    if (hour >= 22) {
        greeting = "All Nighter";
        additional = "?";
    } else if (hour >= 18) {
        greeting = "Good Evening";
    } else if (hour >= 12) {
        greeting = "Good Afternoon";
    } else {
        greeting = "Good Morning";
    }

    return [greeting, additional];
}

function set_greeting_text() {
    let data = get_greeting();
    console.log(data[0], data[1]);
    document.getElementById("greeting_span").textContent = data[0];

    if (typeof data[1] != undefined) {
        document.getElementById("additional_span").textContent = data[1];
    }
}
