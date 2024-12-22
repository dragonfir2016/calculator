use std::io::{self, Write};

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;

    loop {
        println!("\n--- Калькулятор ---");
        println!("Збережений результат: {:.2}", memory);
        println!("Доступні операції:");
        println!("1. Додавання (+)");
        println!("2. Віднімання (-)");
        println!("3. Множення (*)");
        println!("4. Ділення (/)");
        println!("5. Зберегти результат у пам'ять");
        println!("6. Очистити пам'ять");
        println!("7. Вийти");
        print!("Оберіть операцію: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_string();

        if choice == "7" {
            println!("Вихід... До побачення!");
            break;
        }

        let mut num1 = 0.0;
        let mut num2 = 0.0;
        let mut result = 0.0;

        if choice == "1" || choice == "2" || choice == "3" || choice == "4" {
            loop {
                println!("Введіть перше число:");
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).unwrap();
                match num_input.trim().parse() {
                    Ok(val) => {
                        num1 = val;
                        break;
                    }
                    Err(_) => println!("Помилка: введіть коректне число.")
                }
            }

            loop {
                println!("Введіть друге число:");
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).unwrap();
                match num_input.trim().parse() {
                    Ok(val) => {
                        num2 = val;
                        break;
                    }
                    Err(_) => println!("Помилка: введіть коректне число.")
                }
            }
        }

        match choice.as_str() {
            "1" => {
                result = num1 + num2;
                println!("Результат: {:.2}", result);
            }
            "2" => {
                result = num1 - num2;
                println!("Результат: {:.2}", result);
            }
            "3" => {
                result = num1 * num2;
                println!("Результат: {:.2}", result);
            }
            "4" => {
                if num2 == 0.0 {
                    println!("Помилка: ділення на нуль неможливе.");
                    continue;
                }
                result = num1 / num2;
                println!("Результат: {:.2}", result);
            }
            "5" => {
                memory = prev_result;
                println!("Результат збережено в пам'ять: {:.2}", memory);
            }
            "6" => {
                memory = 0.0;
                println!("Пам'ять очищено.");
            }
            _ => {
                println!("Некоректний вибір. Спробуйте ще раз.");
                continue;
            }
        }
        prev_result = result;
    }
}
