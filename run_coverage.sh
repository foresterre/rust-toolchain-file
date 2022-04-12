cargo tarpaulin --out Xml --ignore-tests --exclude-files src/**/tests.rs
pycobertura show --format html --output cov.html cobertura.xml
xdg-open cov.html