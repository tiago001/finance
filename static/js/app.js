load_home()

function load_home(){
    fetch("pages/home.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
    })
    .catch((error) => {
        console.warn(error);
    });
}

function load_add_expense(){
    fetch("pages/add_expense.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
        document.getElementsByClassName("date")[0].valueAsDate = new Date()
        search_last_expenses()
    })
    .catch((error) => {
        console.warn(error);
    });
}

function load_search_expenses(){
    fetch("pages/search_expenses.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
        search_last_expenses();

        for (let i of document.getElementsByClassName("date")) {i.style.display = "none"}
        document.getElementsByClassName("search")[0].addEventListener("change", function(){
            if(document.getElementsByClassName("search")[0].value == "date"){
                document.getElementsByClassName("category")[0].style.display = "none"
                for (let i of document.getElementsByClassName("date")) {i.style.display = "flex"}
            } else if(document.getElementsByClassName("search")[0].value == "category"){
                document.getElementsByClassName("category")[0].style.display = "block"
                for (let i of document.getElementsByClassName("date")) {i.style.display = "none"}
            }
        });
    })
    .catch((error) => {
        console.warn(error);
    });
}

function load_dashboard(){
    fetch("pages/dashboard.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
        fetch("search_expenses_category")
        .then((response) => response.json())
        .then((json) => {
            let label = []
            let sum = []

            json.forEach(e => {
                label.push(e.category)
                sum.push(e.sum)
                document.getElementsByClassName("category_values")[0].insertAdjacentHTML(
                    "beforeend",
                    `
                        <div class="col-6" style=\"border-top: 1px solid #4f4f4f;\">
                            ${e.category} 
                        </div>
                        <div class="col-6 text-center" style=\"border-top: 1px solid #4f4f4f;\">
                            ${e.sum.toFixed(2)} 
                        </div>
                    `
                )
            })

            const ctx = document.getElementById('myChart');

            new Chart(ctx, {
                type: 'doughnut',
                data: {
                    labels: label,
                    datasets: [{
                        label: 'Sum',
                        data: sum,
                        borderWidth: 1
                    }]
                }
            });

        })
        .catch((error) => {
            console.warn(error);
        });
    })
    .catch((error) => {
        console.warn(error);
    });
}

function save_expense(){
    fetch("save_expense?" + new URLSearchParams({
        "name": document.getElementsByClassName("nome")[0].value,
        "value": document.getElementsByClassName("money_value")[0].value,
        "category": document.getElementsByClassName("category")[0].value,
        "date": document.getElementsByClassName("date")[0].value}),
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
        load_add_expense()
        document.getElementsByClassName("expenses").innerHTML = ''
        search_last_expenses()
        setTimeout(() => {
            document.getElementsByClassName("nome")[0].focus();
        }, 200);
    })
    .catch((error) => {
        console.warn(error);
    });
}

function search_last_expenses(){
    fetch("search_last_expenses")
    .then((response) => response.json())
    .then((json) => {
        fill_expenses(json)
    })
    .catch((error) => {
        console.warn(error);
    });
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
            document.getElementsByClassName("expenses")[0].insertAdjacentHTML(
                "beforeend",
                `
                    <div class="col-3" ${e.date != lastDate ? "style=\"border-top: 1px solid #4f4f4f;\"" : ""}>
                        ${e.name} 
                    </div>
                    <div class="col-3 text-center" ${e.date != lastDate ? "style=\"border-top: 1px solid #4f4f4f;\"" : ""}>
                        ${e.value.toFixed(2)} 
                    </div>
                    <div class="col-3 text-center" ${e.date != lastDate ? "style=\"border-top: 1px solid #4f4f4f;\"" : ""}>
                        ${e.category} 
                    </div>
                    <div class="col-3 text-center" ${e.date != lastDate ? "style=\"border-top: 1px solid #4f4f4f;\"" : ""}>
                         ${e.date != lastDate ? e.date : ""}
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
}

function search_expenses(){
    let value1
    if (document.getElementsByClassName("search")[0].value == "category"){
        value1 = document.getElementsByClassName("category_select")[0].value
    } else {
        value1 = document.getElementsByClassName("date1")[0].value
    }

    fetch("search_expenses?" + new URLSearchParams({
        "name": document.getElementsByClassName("search")[0].value,
        "value1": value1,
        "value2": document.getElementsByClassName("date2")[0].value
    }))
    .then((response) => response.json())
    .then((json) => {
        fill_expenses(json)
    })
    .catch((error) => {
        console.warn(error);
    });
}

function search_last_expenses(){
    fetch("search_expenses?" + new URLSearchParams({
        "name": "last15"
    }))
    .then((response) => response.json())
    .then((json) => {
        fill_expenses(json)
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
