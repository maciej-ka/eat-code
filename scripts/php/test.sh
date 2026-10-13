!#/bin/bash
[ ! -d ./vendor ] && composer install
composer test
