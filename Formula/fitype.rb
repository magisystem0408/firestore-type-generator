class Fitype < Formula
  desc "A tool to generate TypeScript types from Firestore schema"
  homepage "https://github.com/magisystem0408/firestore-type-generator"
  version "0.0.1"
  license "MIT"

  if OS.mac?
    url "https://github.com/magisystem0408/firestore-type-generator/releases/download/v#{version}/fitype-aarch64-apple-darwin.tar.gz"
    sha256 "8b8782efa3025e92f0703c077f7342fce0ff1cde4aa957900776cacc956210b4"
  end

  def install
    bin.install "fitype"
  end

  test do
    system "#{bin}/fitype", "--version"
  end
end 