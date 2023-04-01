
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

function load_groups() {
    const path = '/api/groups/all';
    const url = `${window.location.protocol}//${window.location.host}${path}`;

    fetch(url)
        .then(response => response.json())
        .then(data => {
            display_groups(data)
        })
        .catch(error => {
            console.error('Error fetching data:', error);
        });

}
function get_card(name) {
    const path = '/template/card/';
    const url = `${window.location.protocol}//${window.location.host}${path}${name}`;
    fetch(url)
        .then(response => response.text())
        .then(html => {
            let element = document.getElementById("group_row");
            element.innerHTML += html;
        })
        .catch(error => {
            console.error('Error fetching HTML:', error);
        });
}
function display_groups(groups) {
    
    groups.forEach(group => {
        let name = group.name;
        get_card(name);
      });
}

load_groups()