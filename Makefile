.PHONY: docs

docs:
	rm -rf docs; cargo doc && cp -R target/doc ./docs
