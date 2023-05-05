load_home()

function load_home(){
    fetch("pages/home.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
        get_user_info()
    })
    .catch((error) => {
        console.warn(error);
    });
}

function get_user_info(){
    fetch("get_user_info", {redirect: 'follow'})
    .then((response) => {
        if (response.redirected) window.location.href = response.url;
        return response.json()
    })
    .then((json) => {
        console.log(json)
        document.getElementsByClassName("username")[0].innerHTML = json.email
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
    .catch((error) => {
        console.warn(error);
    });
}

function load_dashboard(){
    fetch("pages/dashboard.html")
    .then((response) => response.text())
    .then((html) => {
        document.getElementById("content").innerHTML = html;
        fetch("search_expenses_category", {redirect: 'follow'})
        .then((response) => {
            if (response.redirected) window.location.href = response.url;
            return response.json()
        })
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
    fetch("search_last_expenses", {redirect: 'follow'})
    .then((response) => {
        if (response.redirected) window.location.href = response.url;
        return response.json()
    })
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
                    <div class="col-3" expense="${e.id}" value=${e.name} ${e.date != lastDate ? "style=\"border-top: 1px solid #4f4f4f;\"" : ""}>
                        <span class="name" expense="${e.id}" value=${e.name} contenteditable="true">${e.name}</span> 
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
    }), {redirect: 'follow'})
    .then((response) => {
        if (response.redirected) window.location.href = response.url;
        return response.json()
    })
    .then((json) => {
        fill_expenses(json)
        show_expenses_graph(json)
    })
    .catch((error) => {
        console.warn(error);
    });
}

let graph
function show_expenses_graph(json){
    if(graph){
        graph.destroy()
    }
    let ctx = document.getElementById('graph');


    let datas = []

    for (let index = json.length-1; index >= 0; index--) {
        let e = json[index];
        
        found = false
        datas.forEach(d => {
            if(d.date == e.date){
                d.value = d.value + e.value
                found = true
            }
        });
    
        if(found == false){
            datas.push({"date" : e.date, "value" : e.value})
        }
    }

    let labels = []
    let data = []
    let total = 0
    let acumulative = []
    datas.forEach(d => {
        labels.push(d.date)
        data.push(d.value.toFixed(2))
        total = total + d.value
        acumulative.push(total.toFixed(2))
    })

    graph = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: labels,
        datasets: [ {
            type: 'line',
            label: 'Acumulado',
            data: acumulative,
            yAxisID: 'myScale',
            borderColor: '#ff6384',
            backgroundColor: "#ff6384"
        },{
          label: 'Valor',
          data: data,
          borderWidth: 1,
          borderColor: '#36a2eb',
          backgroundColor: "#36a2eb"
        }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true
          },
          myScale: {
            // type: 'logarithmic',
            position: 'right', // `axis` is determined by the position as `'y'`
          }
        }
      }
    });
}

function search_last_expenses(){
    fetch("search_expenses?" + new URLSearchParams({
        "name": "last15"
    }), {redirect: 'follow'})
    .then((response) => {
        if (response.redirected) window.location.href = response.url;
        return response.json()
    })
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
