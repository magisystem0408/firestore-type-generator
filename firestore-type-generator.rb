class FirestoreTypeGenerator < Formula
  desc "A tool to generate TypeScript types from Firestore schema"
  homepage "https://github.com/magisystem0408/firestore-type-generator"
  version "0.0.1"
  license "MIT"

  if OS.mac?
    url "https://github.com/magisystem0408/firestore-type-generator/releases/download/v#{version}/firegen-aarch64-apple-darwin.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256"

  def install
    bin.install "firegen"
  end

  test do
    system "#{bin}/firegen", "--version"
  end
end 