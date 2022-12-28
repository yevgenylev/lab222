use core::borrow::Borrow;

const BUF_ADDR: u32 = 0xb8000; // адреса початку буферу VGA
const BUF_HEIGHT: u32 = 25; // висота буферу (екрану)
const BUF_WIDTH: u32 = 80;  // ширина буферу (екрану)

const COLOR_BLACK: u8 = 0x0; // константа для чорного кольору

// структура, що репрезентує символ ASCII
pub struct AsciiChar {
    pub char_byte: u8, // значення символу
    pub color_byte: u8 // значення кольору
}

// enum, що зберігає типи вирівнювання
pub enum Alignment {
    Left,
    Right,
    Center
}

// enum, що зберігає значення кольорів
#[repr(u8)]
pub enum Color {
    BLUE = 0x1,
    GREEN = 0x2,
    AZURE = 0x3,
    RED = 0x4,
    PURPLE = 0x5,
    BROWN = 0x6,
    LIGHT_GREY = 0x7,
    DARK_GREY = 0x8,
    LIGHT_BLUE = 0x9,
    LIGHT_GREEN = 0xa,
    LIGHT_AZURE = 0xb,
    LIGHT_RED = 0xc,
    PINK = 0xd,
    YELLOW = 0xe,
    WHITE = 0xf
}

// структура, що зберігає параметри нашого екрану
pub struct Screen {
    buffer: *mut u8, // буфер
    color: u8,       // колір
    align: Alignment, // тип вирівнювання
    cursor_row: u32, // позиція курсору в рядку
    cursor_col: u32, // позиція курсору в стовпчику
    state : [[u8; BUF_WIDTH as usize]; BUF_HEIGHT as usize], // поточний стан екрану
    cursor_write: u32 // кількість заповнених рядків
}

// імплементуємо поведінку макроса write! для структури Screen
impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
// імплементація структури Screen (можна сказати, аналог методів класу в Java чи C++)
impl Screen {
    // конструктор з параметрами, де ми наділяємо поля відповідними значеннями
    pub fn new(color: u8, align: Alignment) -> Screen {
        return Screen {
            buffer: BUF_ADDR as *mut u8,
            color: (COLOR_BLACK << 4) | color,
            align,
            cursor_row: 0,
            cursor_col: 0,
            state : [[0; BUF_WIDTH as usize]; BUF_HEIGHT as usize],
            cursor_write: 0
        }
    }

    // функція, що записує символ до буферу
    pub fn write_char(&mut self, offset: u32, char: AsciiChar) {
        unsafe {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
        }

        self.cursor_write += 1;
    }

    // функція, що зчитує символ з буферу
    pub fn read_char(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }

    // функція для виведення на екран цілого рядку
    pub fn print(&mut self, s: &str) {
        // зберігаємо вхідний рядок до поточного стану екрану
        self.add_text(s.as_bytes());

        // встановлюємо похицію для запису символу на 0 (початок)
        self.cursor_write = 0;
        // йдемо по кожному рядку state, виводимо всі рядки по символу
        for row in self.state {
            /*
                кількість пробілів, що треба поставити перед початком рядка, щоб дотримуватися
                типу вирівнювання
            */
            let align = self.calc_align(&row);

            // вирівнюємо рядок відповідною кількістю пробілів
            for i in 0..align {
                self.write_char(
                    self.cursor_write, AsciiChar{char_byte : b' ', color_byte: self.color}
                );
            }
            // виводимо рядок посимвольно
            for c in row {
                // якщо зустрічаємо знак кінця рядку, припиняємо його виведення
                if c == b'\0' {
                    break;
                }
                // запис символу c на позицію cursor_write
                self.write_char(
                    self.cursor_write, AsciiChar{char_byte : c, color_byte: self.color}
                );
            }

            // переведення курсору запису на новий рядок
            self.cursor_write += BUF_WIDTH - (self.cursor_write % BUF_WIDTH);
        }
    }

    // функція для збереження тексту до поточного стану екрану
    pub fn add_text(&mut self, row: &[u8]) {
        /*
            * записуємо текст посимвольно
            * якщо довжина рядку перевищує ширину екрану або зустрічається символ переходу на новий рядок,
              то переходимо на новий рядок
        */
        for i in 0..row.len() {
            if self.cursor_row == BUF_WIDTH - 1 || row[i] == b'\n' {
                self.state[self.cursor_col as usize ][self.cursor_row as usize] = b'\0';
                self.cursor_col += 1;
                self.cursor_row = 0;

                // якщо рядків стає "забагато"
                // то прогортуємо екран на один рядок
                if self.cursor_col == BUF_HEIGHT - 1 {
                    self.scroll();
                }

                continue;
            }
            self.state[self.cursor_col as usize][self.cursor_row as usize] = row[i];
            self.cursor_row += 1;
        }
    }

    /*
        функція, що зміщує всі рядки на один вгору та затирає останній,
        тим самим звільняючи місце для нового рядка
    */
    pub fn scroll(&mut self) {
        // зміщуємо рядки на один вгору
        for i in 0..self.cursor_col - 1 { // ітеруємося по всіх рядках, крім останнього
            self.state[i as usize] = self.state[(i + 1) as usize]; // зміщуємо всі рядки на 1 вгору
        }

        // затираємо останній рядок
        for i in 0..self.state[self.cursor_col as usize].len() {
            self.state[(self.cursor_col) as usize][i] = b' ';
        }

        self.cursor_col -= 1;
    }

    /*
        функція, що вираховує кількість пробілів,
        необхідну для конкретного типу вирівнювання конкретного рядка
     */
    pub fn calc_align(&self, row: &[u8]) -> u32 {
        let mut len = 0;

        // знаходимо довжину рядка до символу закінчення
        for c in row {
            if *c == b'\0' {
                break;
            }
            len += 1;
        }

        // відповідно до типу, вираховуємо кількість пробілів для вирівнювання
        match self.align {
            Alignment::Left => 0,
            Alignment::Right => BUF_WIDTH - len,
            Alignment::Center => (BUF_WIDTH - len) / 2
        }
    }
}
