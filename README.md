# Simple Decoder

Rust-крейт для **МАКСИМАЛЬНО** лёгкого декодирования и обработки аудио.

## Quick Start

Всё, что нужно для счастья:

```rust
use simple_decoder::SimpleDecoder;
use simple_decoder::decoders::AudioDecoder;

fn main() {
    let data = std::fs::read("melody.mp3").expect("Не удалось прочитать файл");
    let decoder = SimpleDecoder::new();
    
    // Декодируем, ресемплим в 48кГц и переводим в моно одной цепочкой
    let track = decoder.decode(&data).unwrap()
        .resample(48000).unwrap()
        .rechannel(1);
        
    println!("Готово: {} сэмплов", track.pcm.len());
}
```

## Основные сущности

  * **`SimpleDecoder`** — универсальный комбайн. Находится в корне, сам определяет формат файла.
  * **`.decode(&[u8])`** — основной метод, возвращает структуру `AudioTrack`.
  * **`AudioTrack`** — контейнер с сырыми данными (`f32`) и методами трансформации.

-----

## Архитектура проекта

Библиотека построена на базе `symphonia` (декодинг) и `rubato` (ресемплинг).

  * `SimpleDecoderError` — перечисление всех возможных бед.
  * `SimpleDecoder` — точка входа.
  * `audio`:
      * `AudioTrack` — структура с PCM-данными.
      * `AudioFormat` — перечисление форматов (MP3, WAV, FLAC и т.д.).
  * `decoders` (специализированные декодеры):
      * `aac` : `AACDecoder`
      * `flac` : `FLACDecoder`
      * `mp3` : `MP3Decoder`
      * `oggopus` : `OGGOpusDecoder`
      * `oggvorbis` : `OGGVorbisDecoder`
      * `simple` : `SimpleDecoder`
      * `wav` : `WAVDecoder`

-----

## Powerful Transformations

После декодирования доступен **Chain API** для подготовки аудио к воспроизведению:

```rust
let track = decoder.decode(&data)?
    .resample(48000)?  // Высококачественный FFT ресемплинг
    .rechannel(2);     // Изменение количества каналов
```

### Методы `AudioTrack`

| Метод | Описание |
| :--- | :--- |
| `.resample(target_rate)` | Изменяет частоту дискретизации. Использует Sinc-интерполяцию (FFT). |
| `.rechannel(target_channels)` | Изменяет кол-во каналов. Считает среднее при Downmix (Stereo -\> Mono). |
| `.pcm` | Прямой доступ к `Vec<f32>` (Interleaved PCM). |
| `.format` | Возвращает исходный формат файла (`AudioFormat`). |
| `.channels` | Текущее количество каналов (`u16`). |
| `.sample_rate` | Текущая частота дискретизации (`u64`). |

-----

## Лицензия

Максимально просто — [MIT](LICENSE).

