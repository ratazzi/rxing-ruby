# Rxing Ruby

Rxing Ruby is a modern barcode reader library for Ruby, providing high-performance bindings to [rxing](https://github.com/rxing-core/rxing) - a Rust port of the popular ZXing ("Zebra Crossing") barcode scanning library.

With both ZBar and ruby-zbar no longer actively maintained, Rxing Ruby aims to be the go-to solution for barcode reading in Ruby applications. It combines the reliability of ZXing's battle-tested algorithms with the performance benefits of Rust, offering a fresh alternative to legacy solutions like ZBar, ruby-zbar, and other ZXing-based implementations.

Whether you're migrating from ZBar/ruby-zbar or starting a new project, Rxing Ruby provides a clean, efficient, and well-maintained solution for all your barcode scanning needs.

## Features

- Supports multiple barcode formats:
  - 1D: Code 39, Code 93, Code 128, EAN-8, EAN-13, UPC-A, UPC-E, and more
  - 2D: QR Code, Data Matrix, and more
- Read barcodes from files or raw bytes
- Detect multiple barcodes in a single image
- High performance through Rust implementation

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'rxing'
```

And then execute:

```bash
$ bundle install
```

Or install it yourself as:

```bash
$ gem install rxing
```

## Usage

### Reading barcodes from a file

```ruby
reader = Rxing::BarcodeReader.new
results = reader.read_from_file('path/to/your/barcode.png')

results.each do |result|
  puts "Content: #{result.text}"
  puts "Format: #{result.format}"
end
```

### Reading barcodes from bytes

```ruby
reader = Rxing::BarcodeReader.new
image_data = File.read('path/to/your/barcode.png')
results = reader.read_from_bytes(image_data)

results.each do |result|
  puts "Content: #{result.text}"
  puts "Format: #{result.format}"
end
```

## Important Notes

### About Barcode Location Information

While `BarcodeResult` objects include a `points` method that returns the barcode's position in the image, please note:

- Due to limitations inherited from ZXing's algorithms, the returned coordinates may not be entirely accurate, especially for certain 1D barcodes (like Code 39)
- The coordinate information is primarily intended for debugging purposes
- It's not recommended to strictly rely on these coordinate values in production environments
- If you need precise barcode location information, consider using specialized barcode localization libraries

## Development

1. Clone the repository
2. Run `bundle install` to install dependencies
3. Run `rake test` to run the tests

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ratazzi/rxing-ruby.

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

This project contains bindings to the [rxing](https://github.com/rxing-core/rxing) library, which is licensed under the Apache License 2.0. While this Ruby bindings project uses the MIT License, the underlying rxing library remains under the Apache License 2.0.

### License Notice for rxing

The rxing core library is copyright of its respective owners and contributors and is licensed under the Apache License 2.0. See the [rxing repository](https://github.com/rxing-core/rxing) for more details.

