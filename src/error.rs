
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ExtensionMissing, //отсутствует расширение файла
    FailBytes, //ошибка преобразования в двоичный файл
    UnsupportedFormat, //неподдерживаемый формат
    FailParseDocument, //ошибка парсинга документа
    FailConvertFile, //ошибка конвертации файла
    FailHeader, //ошибка создания заголовка конвертированного файла
}