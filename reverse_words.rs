fn main() {
    let input = "snow dog sun"; // Исходная строка

    let reversed = reverse_words(input); // Вызов функции для разворота порядка слов

    // Вывод результата на экран
    println!("Исходная строка: {}", input);
    println!("Перевернутая строка: {}", reversed);
}

// Функция разворота порядка слов в строке
fn reverse_words(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect(); // Разделяем строку на слова по пробелам

    let mut reversed_words = Vec::new(); // Создаем вектор для перевернутых слов

    // Перебираем слова в обратном порядке
    for word in words.iter().rev() {
        reversed_words.push(*word); // Добавляем каждое слово в вектор
    }

    let result = reversed_words.join(" "); // Объединяем перевернутые слова в одну строку с пробелами

    result // Возвращаем результат
}
