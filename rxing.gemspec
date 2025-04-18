require_relative 'lib/rxing/version'

Gem::Specification.new do |spec|
  spec.name          = "rxing"
  spec.version       = Rxing::VERSION
  spec.authors       = ["ratazzi"]
  spec.email         = ["ratazzi.potts@gmail.com"]

  spec.summary       = "Ruby bindings for rxing barcode reader"
  spec.description   = "A Ruby gem that provides bindings to the rxing barcode reading library using Magnus. This gem is licensed under MIT, while the underlying rxing library (https://github.com/rxing-core/rxing) is licensed under the Apache License 2.0."
  spec.homepage      = "https://github.com/ratazzi/rxing-ruby"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 3.0.0"

  # Platform will be set dynamically for precompiled gems
  # For source version, it remains as "ruby"

  spec.files = Dir[
    'lib/**/*.rb',
    'ext/**/*.{rb,rs,toml}',
    'ext/**/src/**/*.rs',
    'README.md',
    'LICENSE',
    'CHANGELOG.md',
    'Gemfile',
    '*.gemspec'
  ]

  # Include precompiled binaries if they exist (for platform gems)
  spec.files += Dir['lib/**/*.{bundle,so,dll}'] if spec.respond_to?(:platform) && spec.platform != "ruby"

  spec.require_paths = ["lib"]

  # Only set extensions for source gems (platform = ruby), not for precompiled platform gems
  spec.extensions = ["ext/rxing/extconf.rb"] if !spec.respond_to?(:platform) || spec.platform == "ruby"

  # Only add development dependencies for source gems
  if !spec.respond_to?(:platform) || spec.platform == "ruby"
    spec.add_development_dependency "rake", "~> 13.0"
    spec.add_development_dependency "rake-compiler", "~> 1.2"
    spec.add_development_dependency "minitest", "~> 5.0"
  end

  # Runtime dependency is needed for both source and precompiled gems
  spec.add_dependency "rb_sys", "~> 0.9.87"

  spec.metadata = {
    "source_code_uri" => "https://github.com/ratazzi/rxing-ruby",
    "bug_tracker_uri" => "https://github.com/ratazzi/rxing-ruby/issues",
    "rubygems_mfa_required" => "true",
    "rxing_library_license" => "Apache-2.0",
    "rxing_library_repo" => "https://github.com/rxing-core/rxing"
  }
end
