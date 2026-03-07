deploy:
	mv _config.yml _config.yml.tmp
	cp _buildconfig.yml _config.yml
	rm -rf docs
	arch -x86_64 jekyll build 
	rm -rf _site/docs
	cp -r _site docs
	git add docs
	git commit -m "webpage update"
	git push origin main
	mv _config.yml.tmp _config.yml

.PHONY: deploy
