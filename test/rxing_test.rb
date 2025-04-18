# frozen_string_literal: true

require "test_helper"

class RxingTest < Minitest::Test
  FIXTURES_DIR = File.expand_path("fixtures", __dir__)

  def setup
    @reader = Rxing::BarcodeReader.new
  end

  def test_version
    refute_nil ::Rxing::VERSION
  end

  def test_read_code128_from_file
    results = @reader.read_from_file(File.join(FIXTURES_DIR, "code128.png"))
    assert_equal 1, results.size

    result = results.first
    assert_equal "CODE_128", result.format
    assert_equal "HELLO-RXING", result.text
    assert_kind_of Array, result.points
    assert_equal 2, result.points.size
  end

  def test_read_code39_from_file
    results = @reader.read_from_file(File.join(FIXTURES_DIR, "code39.png"))
    assert_equal 1, results.size

    result = results.first
    assert_equal "CODE_39", result.format
    assert_equal "HELLO-RXING", result.text
    assert_kind_of Array, result.points
    assert_equal 2, result.points.size
  end

  def test_read_from_bytes
    file_path = File.join(FIXTURES_DIR, "code128.png")
    image_data = File.read(file_path, mode: "rb")
    results = @reader.read_from_bytes(image_data)

    assert_equal 1, results.size
    result = results.first
    assert_equal "CODE_128", result.format
    assert_equal "HELLO-RXING", result.text
  end

  def test_invalid_file_path
    assert_raises(RuntimeError) do
      @reader.read_from_file("nonexistent.png")
    end
  end

  def test_invalid_image_data
    assert_raises(RuntimeError) do
      @reader.read_from_bytes("invalid image data")
    end
  end

  def test_read_qrcode_from_file
    results = @reader.read_from_file(File.join(FIXTURES_DIR, "qrcode.png"))
    assert_equal 1, results.size

    result = results.first
    assert_equal "QR_CODE", result.format
    assert_equal "https://github.com/ratazzi/rxing-ruby", result.text
    assert_kind_of Array, result.points
    assert_equal 4, result.points.size
  end
end
