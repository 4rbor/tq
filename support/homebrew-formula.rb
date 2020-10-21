class Tq < Formula
  desc "Command line TOML Processor"
  homepage "https://github.com/4rbor/tq"
  version "_version_"

  if OS.mac?
    url "_mactar_"
    sha256 "_macsha_"
  elsif OS.linux?
    url "_lintar_"
    sha256 "_linsha_"
  end

  def install
    bin.install "tq"
  end
end
