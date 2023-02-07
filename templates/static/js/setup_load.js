function on_load() {
    // check that the local database has been 
    const url = location.origin + "/api/database_exists";
    $.get(url, function(data, _status) {
        let name = data['name'];
        let value = data['value'];


    })   
}

let allow_submit = [false, false];

$("#name_input").keyup(function(){
    allow_submit[0] = !validate_input(this)
    update_button()
})

$("#email_input").keyup(function(){
    allow_submit[1] = !validate_input(this)
    update_button()
})

function validate_input(ix) {
    return ix.value == ''
}

function update_button() {
    if (allow_submit[0] == true && allow_submit[1] == true) {
        $("#submit_form").prop("disabled", false)
    } else {
        $("#submit_form").prop("disabled", true)
    }
}

function submit_setup_form() {
    let form_data = {
        "name": $("#name_input").val(),
        "email": $("#email_input").val(),
        "path": $("#sql_path_input").val(),

    };

    const url = location.origin + "/setup";
    $.ajax({
        type: "POST",
        url: url,
        data: form_data,
        success: function(){
            console.log("Yay")
        },
        dataType: "json",
        contentType : "application/json"
      });

}