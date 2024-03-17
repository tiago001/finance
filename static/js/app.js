let graph



function delete_expense(id){
    
    fetch("delete_expense?" + new URLSearchParams({
        "id": id}), {method: "POST", redirect: 'follow'})
        .then((response) => {
            if (response.redirected) window.location.href = response.url;
            return
        })
        .then(() => {
            search_expenses()
            Swal.fire({
                position: 'top-end',
                icon: 'success',
                title: 'Despesa deletada com sucesso',
                showConfirmButton: false,
                timer: 2000,
                toast: true
            })
        })
        .catch((error) => {
            console.warn(error);
        });

}

function open_edit_expense(id){
    var exampleModal = document.getElementById('exampleModal')
    var modalTitle = exampleModal.querySelector('.modal-title')

    modalTitle.textContent = 'Editar despesa'

    $(".modal-body").load("editexpense", function() {
        fetch("get_expense?" + new URLSearchParams({
            "id": id
        }), {redirect: 'follow'})
        .then((response) => {
            if (response.redirected) window.location.href = response.url;
            return response.json()
        })
        .then((json) => {
            document.getElementsByClassName("editnome")[0].value = json.name
            document.getElementsByClassName("editmoney_value")[0].value = json.value
            document.getElementsByClassName("editcategory")[0].value = json.category
            document.getElementsByClassName("editdate")[0].value = json.date
        })
        .catch((error) => {
            console.warn(error);
        });
    })
    
    $(".modal .btn-primary")[0].setAttribute('onclick',`edit_expense(${id})`)
    $(".modal .btn-danger")[0].setAttribute('onclick',`delete_expense(${id})`)
}

function edit_expense(id){
    fetch("edit_expense?" + new URLSearchParams({
        "id": id,
        "name": document.getElementsByClassName("editnome")[0].value,
        "value": document.getElementsByClassName("editmoney_value")[0].value,
        "category": document.getElementsByClassName("editcategory")[0].value,
        "date": document.getElementsByClassName("editdate")[0].value}),
    {
        method: "POST"
    })
    .then((response) => response.text())
    .then(() => {
        search_expenses()
        Swal.fire({
            position: 'top-end',
            icon: 'success',
            title: 'Despesa alterada com sucesso',
            showConfirmButton: false,
            timer: 2000,
            toast: true
        })
    })
    .catch((error) => {
        console.warn(error);
    });
}

document.addEventListener("keypress", function(event) {
    if (event.key === "Enter") {
        save_expense();
    }
});

function loadPage(href, pushState){
    // const requestOptions = {
    //     method: "GET",
    //     redirect: "follow"
    // };

    let url = href.replace(window.location.protocol + "//" + window.location.host, "");

    if(pushState){
        window.history.pushState(null, "Projeto Finanças", url)
    }
    $("#content").load(href)

    // fetch(href, requestOptions)
    //     .then((response) => response.text())
    //     .then((result) => {
    //         document.getElementById("content").innerHTML = result
    //         console.log(result)
    //     })
    //     .catch((error) => console.error(error));
}

document.onclick = function (e) {
    e = e ||  window.event;
    var element = e.target || e.srcElement;

    var tagName = element.tagName

    if(tagName == 'BUTTON' || tagName == 'P' || tagName == 'I' || tagName == 'SPAN'){ 
        element = element.parentElement 
    }
    // if(element.tagName == 'P'){ element = element.parentElement }
    // if(element.tagName == 'I'){ element = element.parentElement }
    // if(element.tagName == 'SPAN'){ element = element.parentElement }

    if (element.tagName == 'A') {
        let url = element.href;
        if(url != window.location){
            loadPage(element.href, true);
        }
        return false; // prevent default action and stop event propagation
    }
};

window.addEventListener("popstate", function(e) {
    loadPage(location.href, false)
});