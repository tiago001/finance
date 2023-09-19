let graph

// load_home()

function load_home(){
    $("#content").load("pages/home.html", function() {
        // get_user_info()
    })
}

// function get_user_info(){
//     fetch("get_user_info", {redirect: 'follow'})
//     .then((response) => {
//         if (response.redirected) window.location.href = response.url;
//         return response.json()
//     })
//     .then((json) => {
//         document.getElementById("username").innerHTML = json.name
//     })
//     .catch((error) => {
//         console.warn(error);
//     });
// }

function load_add_expense(){
    // window.history.pushState("","Add expenses", "/addexpenses");

    $("#content").load("pages/add_expense.html", function() {
        let date = new Date();

        document.getElementsByClassName("date")[0].valueAsDate = new Date(date.getFullYear(), date.getMonth(), date.getDate())
        search_last_expenses()
    })
}

function load_search_expenses(){
    // window.history.pushState("","Search expenses", "/searchexpenses");

    $("#content").load("pages/search_expenses.html", function() {
        var today = new Date();
        var lastDayOfMonth = new Date(today.getFullYear(), today.getMonth()+1, 0);
        var firstDayOfMonth = new Date(today.getFullYear(), today.getMonth(), 1);
        
        document.getElementsByClassName("date1")[0].valueAsDate = firstDayOfMonth
        document.getElementsByClassName("date2")[0].valueAsDate = lastDayOfMonth
        
        for (let i of document.getElementsByClassName("category")) {i.style.display = "none"}
        document.getElementsByClassName("search")[0].addEventListener("change", function(){
            if(document.getElementsByClassName("search")[0].value == "date"){
                document.getElementsByClassName("category")[0].style.display = "none"
                for (let i of document.getElementsByClassName("date")) {i.style.display = "flex"}
            } else if(document.getElementsByClassName("search")[0].value == "category"){
                document.getElementsByClassName("category")[0].style.display = "block"
                for (let i of document.getElementsByClassName("date")) {i.style.display = "none"}
            }
        });

        search_expenses()
    })
}

function load_dashboard(){
    $("#content").load("pages/dashboard.html", function() {
        var today = new Date();
        var lastDayOfMonth = new Date(today.getFullYear(), today.getMonth()+1, 0);
        var firstDayOfMonth = new Date(today.getFullYear(), today.getMonth()-2, 1);
    
        document.getElementsByClassName("date1")[0].valueAsDate = firstDayOfMonth
        document.getElementsByClassName("date2")[0].valueAsDate = lastDayOfMonth

        search_expenses_category()
    })
}

function fill_expenses(json){
    document.getElementsByClassName("expenses")[0].innerHTML = ''
    document.getElementsByClassName("expenses")[0].insertAdjacentHTML("beforeend",
        `
            <div class="col-3">
                Nome
            </div>
            <div class="col-3 text-center">
                Valor
            </div>
            <div class="col-3 text-center">
                Categoria
            </div>
            <div class="col-3 text-center">
                Data
            </div>
        `
        )
        let total = 0
        let lastDate = ""
        json.forEach(e => {
            total += e.value
            // total = parseFloat(total.toFixed(10))
            document.getElementsByClassName("expenses")[0].insertAdjacentHTML(
                "beforeend",
                `
                    <div class="col-3" expense="${e.id}" value=${e.name} ${e.date != lastDate ? "style=\"border-top: 2px solid #e1e1e1;\"" : ""}>
                        <span class="name" expense="${e.id}" value=${e.name} contenteditable="true">${e.name}</span> 
                    </div>
                    <div class="col-3 text-center" expense="${e.id}" ${e.date != lastDate ? "style=\"border-top: 2px solid #e1e1e1;\"" : ""}>
                        <span class="valor" expense="${e.id}" value="${e.value.toFixed(2)}" contenteditable="true">${e.value.toFixed(2)}</span>
                    </div>
                    <div class="col-3 text-center" ${e.date != lastDate ? "style=\"border-top: 2px solid #e1e1e1;\"" : ""}>
                        ${e.category} 
                    </div>
                    <div class="col-2 text-center" ${e.date != lastDate ? "style=\"border-top: 2px solid #e1e1e1;\"" : ""}>
                         ${e.date != lastDate ? e.date : ""}
                    </div>
                    <div class="col-1 text-center" ${e.date != lastDate ? "style=\"border-top: 2px solid #e1e1e1;\"" : ""}>
                        <!--<button class="btn btn-sm btn-outline-danger py-0" onclick="delete_expense(${e.id})">
                            <span><i class="ti ti-x"></i></span>
                        </button>-->
                        <button class="btn btn-sm btn-light py-0" onclick="open_edit_expense(${e.id})" data-bs-toggle="modal" data-bs-target="#exampleModal">
                            <span><i class="ti ti-pencil"></i></span>
                        </button>
                    </div>
                `
            )

            lastDate = e.date
        })
        document.getElementsByClassName("expenses")[0].insertAdjacentHTML("afterbegin",
        `
            <div class="col-3"></div>
            <div class="col-3"></div>
            <div class="col-3"></div>
            <div class="col-3 text-center">
                Total ${total.toFixed(2)}
            </div>
        `
        )

        $('.name').on('blur', function(e){
            if(e.target.textContent.trim() != e.target.getAttribute("value")) {
                e.target.setAttribute("value", e.target.textContent.trim()) 
                fetch("edit_expense?" + new URLSearchParams({
                    "name": e.target.textContent.trim(),
                    "id": e.target.getAttribute("expense")}),
                {
                    method: "POST"
                })
                .then((response) => response.text())
                .then((html) => {
                    Swal.fire({
                        position: 'top-end',
                        icon: 'success',
                        title: 'Despesa salva com sucesso',
                        showConfirmButton: false,
                        timer: 2000,
                        toast: true
                      })
                })
                .catch((error) => {
                    console.warn(error);
                });
            }
        })
        $('.valor').on('blur', function(e){
            if(e.target.textContent.trim() != e.target.getAttribute("value")) {
                e.target.setAttribute("value", e.target.textContent.trim()) 
                fetch("edit_expense?" + new URLSearchParams({
                    "value": e.target.textContent.trim(),
                    "id": e.target.getAttribute("expense")}),
                {
                    method: "POST"
                })
                .then((response) => response.text())
                .then((html) => {
                    Swal.fire({
                        position: 'top-end',
                        icon: 'success',
                        title: 'Despesa salva com sucesso',
                        showConfirmButton: false,
                        timer: 2000,
                        toast: true
                      })
                })
                .catch((error) => {
                    console.warn(error);
                });
            }
        })
}

function delete_expense(id){
    
    fetch("delete_expense?" + new URLSearchParams({
        "id": id}), {method: "POST", redirect: 'follow'})
        .then((response) => {
            if (response.redirected) window.location.href = response.url;
            return
        })
        .then(() => {
            load_add_expense()
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
    console.log("edit_expense")
    var exampleModal = document.getElementById('exampleModal')
    var modalTitle = exampleModal.querySelector('.modal-title')

    modalTitle.textContent = 'Editar despesa'

    $(".modal-body").load("pages/edit_expense.html", function() {
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

            console.log(json)
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
        load_add_expense()
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