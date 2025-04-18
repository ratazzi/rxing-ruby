require "bundler/gem_tasks"
require "rake/testtask"
require "rake/extensiontask"
require "rake_compiler_dock"

task default: :test

Rake::ExtensionTask.new("rxing") do |ext|
  ext.lib_dir = "lib/rxing"

  # Add cross-platform compilation support
  ext.cross_platform = ['x86_64-linux', 'x86_64-darwin', 'arm64-darwin', 'x64-mingw32', 'x64-mingw-ucrt']
  ext.cross_compile = true
end

ENV['RB_SYS_CARGO_PROFILE'] = 'release'

# Add 'native' as an alias for 'compile' for compatibility
task :native => :compile

# Add 'gem' as an alias for 'build' for compatibility
task :gem => :build

# Task to create a platform-specific precompiled gem
desc "Compile the extension and build a platform-specific gem"
task :platform_gem do
  require 'rubygems/platform'
  require_relative 'lib/rxing/version'

  # Compile the extension
  Rake::Task["compile"].invoke

  # Get the current platform
  current_platform = Gem::Platform.local.to_s

  # Create a temporary gemspec file with platform set
  gemspec_content = File.read('rxing.gemspec')
  platform_gemspec_content = gemspec_content.gsub(
    /(Gem::Specification\.new do \|spec\|)/,
    "\\1\n  spec.platform = '#{current_platform}'"
  )

  # Ensure GitHub Packages metadata is present for when gems are pushed there
  unless platform_gemspec_content.include?('allowed_push_host')
    platform_gemspec_content = platform_gemspec_content.gsub(
      /(spec\.metadata = {)/,
      "\\1\n    \"github_repo\" => \"#{ENV['GITHUB_REPOSITORY'] || 'ratazzi/rxing-ruby'}\","
    )
  end

  # Write the temporary gemspec
  File.write('rxing_platform.gemspec', platform_gemspec_content)

  # Build the platform-specific gem
  gem_file = "rxing-#{Rxing::VERSION}-#{current_platform}.gem"
  mkdir_p 'pkg'
  sh "gem build rxing_platform.gemspec --output=pkg/#{gem_file}"

  # Remove the temporary gemspec
  rm 'rxing_platform.gemspec'

  puts "Platform-specific gem created: pkg/#{gem_file}"
end

# Task to create renamed gem copies for GitHub Release display
desc "Create renamed copies of gems with Ruby version in the filename"
task :release_gems, [:ruby_version] => [:platform_gem] do |t, args|
  require_relative 'lib/rxing/version'

  unless args[:ruby_version]
    STDERR.puts "Error: Ruby version is required, e.g., rake release_gems[3.2]"
    exit 1
  end

  ruby_version = args[:ruby_version]

  # Create directory for release gems
  mkdir_p 'release_gems'

  # Get all gem files in pkg directory
  gem_files = Dir.glob('pkg/*.gem')

  gem_files.each do |gem_file|
    base_name = File.basename(gem_file)
    new_name = base_name.sub(/\.gem$/, "-ruby-#{ruby_version}.gem")
    cp gem_file, "release_gems/#{new_name}"
    puts "Created renamed copy: release_gems/#{new_name}"
  end
end

# Task to display gem details
desc "Display details of gems in the specified directory"
task :display_gem_info, [:directory] do |t, args|
  directory = args[:directory] || 'pkg'

  puts "Gem details in #{directory}:"
  Dir.glob("#{directory}/*.gem").each do |gemfile|
    puts "=" * 50
    puts "Package: #{gemfile}"
    sh "gem spec \"#{gemfile}\" | grep -E 'name|version|platform|extensions'" do |ok, _|
      # Ignore failure
    end
    puts ""
  end
end

Rake::TestTask.new(:test) do |t|
  t.deps << :compile
  t.libs << "test"
  t.test_files = FileList[File.expand_path("test/*_test.rb", __dir__)]
end

# Add cross-platform precompilation tasks
namespace :build do
  desc "Build gem for all platforms"
  task :all do
    # Ensure we have all Ruby versions
    %w[3.0 3.1 3.2 3.3].each do |ruby_version|
      RakeCompilerDock.sh <<-EOT, platform: 'x86_64-linux', rubyvm: "ruby-#{ruby_version}"
        bundle && bundle exec rake compile && bundle exec rake build
      EOT
    end

    # macOS platforms need to be compiled in a macOS environment
    puts "Note: For macOS platforms (x86_64-darwin, arm64-darwin), please run 'rake compile && rake build' in a macOS environment"

    # Windows platforms
    %w[3.0 3.1 3.2 3.3].each do |ruby_version|
      RakeCompilerDock.sh <<-EOT, platform: 'x64-mingw32', rubyvm: "ruby-#{ruby_version}"
        bundle && bundle exec rake compile && bundle exec rake build
      EOT
    end
  end
end
