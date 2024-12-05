let graph
let graphBalance

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
            document.getElementsByClassName("editmoney_value")[0].value = parseFloat(json.value).toFixed(2)
            document.getElementsByClassName("editcategory")[0].value = json.category_id
            document.getElementsByClassName("editdate")[0].value = json.date
        })
        .catch((error) => {
            console.warn(error);
        });
    })

    $(".modal-footer .btn-danger").show()
    $(".modal-footer .btn-primary").show()
    
    $(".modal .btn-primary")[0].setAttribute('onclick',`edit_expense(${id})`)
    $(".modal .btn-danger")[0].setAttribute('onclick',`delete_expense(${id})`)
}

// function edit_expense(id){
//     fetch("edit_expense?" + new URLSearchParams({
//         "id": id,
//         "name": document.getElementsByClassName("editnome")[0].value,
//         "value": document.getElementsByClassName("editmoney_value")[0].value,
//         "category_id": document.getElementsByClassName("editcategory")[0].value,
//         "date": document.getElementsByClassName("editdate")[0].value}),
//     {
//         method: "POST"
//     })
//     .then((response) => response.text())
//     .then(() => {
//         search_expenses()
//         Swal.fire({
//             position: 'top-end',
//             icon: 'success',
//             title: 'Despesa alterada com sucesso',
//             showConfirmButton: false,
//             timer: 2000,
//             toast: true
//         })
//     })
//     .catch((error) => {
//         console.warn(error);
//     });
// }

function mensagemErro(mensagem){
    Swal.fire({
        position: 'top-end',
        icon: 'error',
        title: mensagem,
        showConfirmButton: false,
        timer: 2500,
        toast: true
      })
}

function searchInvestment(event){
    $(".stocks-list").load("search_investment?"+ new URLSearchParams({
        "stock": document.getElementsByClassName("stock_name")[0].value
    }))
}

document.addEventListener("keypress", function(event) {
    if (event.key === "Enter") {
        page = window.location.toString().replace(window.location.protocol + "//" + window.location.host+"/", "")
        if(page == "addexpenses") {
            save_expense();
        } else if(page == "income"){
            save_income();
        }
    }
});

$.ajaxSetup({
    beforeSend: function(xhr) {
        xhr.setRequestHeader('load-mode', 'not-extended');
    }
});

function loadPage(href, pushState){
    let url = href.replace(window.location.protocol + "//" + window.location.host, "");

    if(pushState){
        window.history.pushState(null, "Projeto Finanças", url)
    }
    
    $("#content").load(href, function(e) {
        if(e !== undefined && e.includes("/verifyaccount")){ // Verify if user was logged out
          window.location.replace(window.location.protocol + "//" + window.location.host+"/login")
        }
    })

    attSideBarMenu()
}

document.onclick = function (e) {
    e = e ||  window.event;
    var element = e.target || e.srcElement;

    var tagName = element.tagName

    if(tagName == 'BUTTON' || tagName == 'P' || tagName == 'I' || tagName == 'SPAN'){ 
        element = element.parentElement 
    }

    if (element != null && element.tagName == 'A') {
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

function attSideBarMenu(){
    var url = window.location + "";

    $("#sidebarnav a").each(function () {
    if($(this).hasClass("active")){
        $(this).removeClass("active");
    }
    if(this.href == url){
        $(this).addClass("active");
    }
    })
}

let installPrompt = null;
const installButton = document.querySelector("#install");

window.addEventListener("beforeinstallprompt", (event) => {
    console.log(event)
    event.preventDefault();
    installPrompt = event;
    installButton.removeAttribute("hidden");
});