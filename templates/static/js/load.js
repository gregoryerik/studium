
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


// instead of making a request for each group. get a tempalte to js then copy and paste onto dom with data from browser
// takes away requests

// get the template

async function get_card_template() {
    const path = '/template/ecard';
    const url = `${window.location.protocol}//${window.location.host}${path}${name}`;

    try {
        let resp = await fetch(url);
        let data = await resp.text();

        return data;

    } catch (error) {
        console.log(error)
    }
}

async function get_groups() {
    const path = '/api/groups/all';
    const url = `${window.location.protocol}//${window.location.host}${path}`;

    try {
        let resp = await fetch(url);
        let data = await resp.json();

        return data;
    } catch (error) {
        console.log(error)
    }
}

async function display_cards_from_template() {
    let template = await get_card_template();
    let data = await get_groups();
    let root_element = document.getElementById("group_row");

    data.forEach(group => {
        let current_template = template;

        let current_element = document.createElement('div');
        current_element.innerHTML = current_template;

        let title_element = current_element.querySelector('#subject_frame_title');
        title_element.innerHTML = group.name;

        root_element.innerHTML += current_element.innerHTML;
    });

}



window.onload = async function () {
    await display_cards_from_template();
}

