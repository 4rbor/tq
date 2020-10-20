class Tq < Formula
  desc "Command line TOML Processor"
  homepage "https://github.com/4rbor/tq"
  version "_version_"

  if OS.mac?
    url "_tarball_"
    sha256 "_sha256_"
  elsif OS.linux?
    url "_linux_tarball_"
    sha256 "_linux_sha256_"
  end

  def install
    bin.install "tq"
  end
end
