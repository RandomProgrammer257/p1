use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};

fn main() {
    const SIZE: (usize, usize) = (5, 5);
    let mut pos: (usize, usize) = (0, 0);

    let mut matrix: [[i32; SIZE.0]; SIZE.1] = [[0; SIZE.0]; SIZE.1];

    let device_state = DeviceState::new();

    println!("Слежение за глобальными нажатиями. Нажмите Ctrl+C в терминале для остановки.");

    loop {
        // Получаем список всех нажатых в данный момент клавиш
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.len() != 0 {
            match keys[0] {
                Keycode::W => go_up(&mut pos, SIZE),
                Keycode::S => go_down(&mut pos, SIZE),
                Keycode::A => go_left(&mut pos, SIZE),
                Keycode::D => go_right(&mut pos, SIZE),

                Keycode::Escape => {
                    println!("Выход из программы...");
                    return; // Завершаем программу
                }
                _ => println!("Нажата другая клавиша: {:?}", keys), // Обязательная ветка для остальных
            }

            matrix = [[0; SIZE.0]; SIZE.1];
            matrix[pos.0][pos.1] = 1;
            
            clearscreen::clear().expect("Ошибка очистки экрана");
            for row in 0..matrix.len() {
                for col in 0..matrix[row].len() {
                    print!("{} ", matrix[row][col]);
                }
                println!(); // Переход на новую строку
            }
            println!();

            // Небольшая задержка, чтобы не нагружать процессор
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
fn go_up(tup: &mut (usize, usize), ranges: (usize, usize)) {
    if tup.0 > 0 {
        tup.0 -= 1; // Изменяем первый элемент
    }
}
fn go_down(tup: &mut (usize, usize), ranges: (usize, usize)) {
    if tup.0 < ranges.0-1 {
        tup.0 += 1; // Изменяем первый элемент
    }
}
fn go_left(tup: &mut (usize, usize), ranges: (usize, usize)) {
    if tup.1 > 0 {
        tup.1 -= 1; // Изменяем первый элемент
    }
}
fn go_right(tup: &mut (usize, usize), ranges: (usize, usize)) {
    if tup.1 < ranges.1-1 {
        tup.1 += 1; // Изменяем первый элемент
    }
}
