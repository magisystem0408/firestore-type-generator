class Fitype < Formula
  desc "A tool to generate TypeScript types from Firestore schema"
  homepage "https://github.com/magisystem0408/firestore-type-generator"
  version "0.0.1"
  license "MIT"

  if OS.mac?
    url "https://github.com/magisystem0408/firestore-type-generator/releases/download/v#{version}/fitype-aarch64-apple-darwin.tar.gz"
    sha256 "27b9ec6088d75947969127c89e3371d2450d9e8162d4d35f4c800a57c258e660"
  end

  def install
    bin.install "fitype"
  end

  test do
    system "#{bin}/fitype", "--version"
  end
end 