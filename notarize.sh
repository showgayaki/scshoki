#!/bin/bash
# 環境変数読み込み
set -a
source .env
set +a

APP_NAME="scshoki"
# バージョンを取得
VERSION=$(cat src-tauri/tauri.conf.json | jq -r '.version')
# アーキテクチャを取得
if [ "$(uname -m)" = "arm64" ]; then
    ARCH="aarch64"
fi
# notarization用のDMGパスを更新
DMG_PATH="src-tauri/target/release/bundle/dmg/${APP_NAME}_${VERSION}_${ARCH}.dmg"

echo "🔹 Signing the app..."
codesign --deep --force --verbose --options runtime \
--sign $APPLE_SIGNING_IDENTITY \
"src-tauri/target/release/bundle/macos/${APP_NAME}.app"

echo "🔹 Submitting to Apple Notary..."
xcrun notarytool submit "${DMG_PATH}" \
--keychain-profile "notarytool-profile" --wait

echo "🔹 Stapling notarization ticket..."
xcrun stapler staple "${DMG_PATH}"

echo "✅ Done! The app is now notarized."
