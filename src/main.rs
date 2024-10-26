#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::{Redirect, content::RawHtml};
use rocket::serde::{Serialize, Deserialize};

#[derive(FromForm)]
struct CalcInput {
    num1: f64,
    num2: f64,
    operation: String,
}

fn perform_operation(op: &str, a: f64, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Ділення на нуль неможливе".to_string())
            } else {
                Ok(a / b)
            }
        },
        _ => Err("Невідома операція".to_string()),
    }
}

#[post("/calculate", data = "<calc_form>")]
fn calculate(calc_form: Form<CalcInput>) -> RawHtml<String> {
    let num1 = calc_form.num1;
    let num2 = calc_form.num2;
    let operation = &calc_form.operation;

    let result = match perform_operation(operation, num1, num2) {
        Ok(res) => res,
        Err(e) => {
            return RawHtml(format!("<p>Помилка: {}</p>", e));
        },
    };

    // Повертаємо HTML для оновлення першого числа з результатом
    RawHtml(format!("<script>document.getElementById('num1').value = {};</script><p>Результат: {}</p>", result, result))
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <html>
    <head>
        <title>Простий калькулятор</title>
        <script>
            function handleSubmit(event) {
                event.preventDefault(); // Запобігаємо перезавантаженню сторінки
                const form = event.target;
                const formData = new FormData(form);

                fetch(form.action, {
                    method: 'POST',
                    body: formData
                })
                .then(response => response.text())
                .then(html => {
                    document.getElementById('result').innerHTML = html;

                    // Оновлюємо значення поля num1
                    const resultMatch = html.match(/Результат: ([0-9.-]+)/);
                    if (resultMatch) {
                        const newValue = resultMatch[1];
                        document.getElementById('num1').value = newValue;
                    }
                });
            }
        </script>
    </head>
    <body>
        <h1>Калькулятор</h1>
        <form action='/calculate' method='post' onsubmit="handleSubmit(event)">
            <input type='number' step='any' name='num1' id='num1' placeholder='Перше число' required>
            <select name='operation'>
                <option value='+'>+</option>
                <option value='-'>-</option>
                <option value='*'>*</option>
                <option value='/'>/</option>
            </select>
            <input type='number' step='any' name='num2' placeholder='Друге число' required>
            <button type='submit'>Обчислити</button>
        </form>
        <div id='result'></div>
    </body>
    </html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, calculate])
}
