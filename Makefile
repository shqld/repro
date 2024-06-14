_init:
	corepack enable
	pnpm install
	pnpm config set store-dir ./.pnpm-store
	cd src; ln -sfnv ./index.js ./linked.js
	cd packages/a/src; ln -sfnv ./index.js ./linked.js

pack:
	pnpm pack --pack-destination ./out/
	tar -xvzf ./out/repro-1.0.0.tgz -C ./out/
	ls -l out/package/src

deploy:
	rm -rf ./out/deployed
	pnpm deploy --filter a ./out/deployed
	ls -l out/deployed/src
	