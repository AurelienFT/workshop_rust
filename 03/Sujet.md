# Sujet 03 - Structures et traits

Votre connaissance du RUST commence a être un peu plus étoffé. Lors de ce workshop nous allons parlés des structures et toutes les fonctionnalités autour. Le RUST étant un language moderne il comporte des fonctionnalités communes à certains autres languages modernes.

## 1. Le mot-clé `struct`

Pour commencez déclarez une structure `Human` cette structure devra avoir 3 champs : `age` de type `i32`, `name` de type `String` et `situation` de type `String`.

*[Documentation RUST structure](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)*

## 2. Le mot clé `enum`

En plus des structures le Rust à aussi un mot-clé pour définir une énumération. Pour faciliter le traitement de données de notre structure `Human`, créez une énumeration `Situation` avec comme valeurs possibles : `EMPLOYEE`, `STUDENT`, `KID`, `BOSS` et changez le champ `situation` pour qu'il soit du type `Situation`.

*[Documentation RUST énumération](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)*

## 3. Utilisation des structures

Maintenant que nous avons défini une structure il est temps de l'utiliser.

Créez une fonction `is_adult()` qui prend en paramètre une structure de type `Human` est qui (en comparant son âge) renvoi un booléen à `True` si l'humain à plus de 18 ans sinon `False`.

## 4. Pattern matching

Nous pouvons déterminer si un humain est majeur en france. Nous aimerions savoir combien dépenses chaque humain en moyenne par mois. Nous avons une petite base de données :

| Kid | Student | Employee | Boss |
|-----|---------|----------|------|
| 0   | 700     | 1500     | 3000 |

En utilisant le "pattern matching" du RUST créez une fonction `get_spending_cost()` qui prend en paramètre un `Human` et qui renvoi la somme qu'il dépense par mois en `i32`

*[Documentation sur le "patern matching"](https://doc.rust-lang.org/book/ch06-02-match.html)*

## 5. Les méthodes

Le Rust permet d'ajouter des méthodes aux structures. Cela tombe bien car les deux fonctions que nous venons de créer sont propres aux humains alors nous pouvons les transformer en méthode pour la structure `Human` non ? Alors faites-le :)

*[Documentation sur les méthodes](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)*

## 6. Les traits

Le RUST propose aussi de la généricité dans la création des méthodes. Nous voulons étendre notre petit programme de d'analyse d'informations en rajoutant les animaux.

Ajoutez une structure `Animal` qui a comme champ : `age` de type `i32`, `name` de type `String`.

Maintenant que nous avons des animaux nous aimerions savoir si ils sont adultes eux aussi, sauf que c'est une méthode de la structure `Human`. On ne peut pas juste dire qu'ils sont adultes à 18 ans, en effet pour eux c'est 7 ans (il faillait un chiffre pour l'exercice).

Créez un trait `LivingBeings` qui contient la méthode `is_adult()` et implémentez là pour les structures `Human` et pour `Animal`. Implémentez aussi un comportement par défaut qui renvoi 0.

*[Documentation sur les traits](https://doc.rust-lang.org/book/ch10-02-traits.html)*

## 7. Les proc-macros

Les proc-macros en Rust sont utilisés sur les structure pour implémenter beaucoup de traits directement sans que l'utilisateur est tous à les définir.

*[Documentation sur les proc-macros](https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=macros#how-to-write-a-custom-derive-macro)*

Créez une proc macro `Beings` avec comme "paramètre" l'age adulte de l'espece et implémentez automatiquement le trait `LivingBeings` avec la valeur passé en paramètre. (en utilisant syn et quote)
