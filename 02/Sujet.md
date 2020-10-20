# Sujet 02 - Erreurs, librairie standard et l'environnement du RUST

Maintenant que vous connaissez les fonctionnalités de bases du Rust nous allons pouvoir utiliser des fonctionnalités propres au RUST et qui facilite la vie.

## 1. Result

Pour renvoyer une erreur chaque language a sa façon de faire. En JS, throw ou renvoyé deux variables une avec le résultat l'autre avec l'erreur. En C++ pour chaque type il y a une valeur comme considérée comme valeur d'erreur. Et bien en RUST il y a le `Result`.

Result est un type templaté dans grace auquel vous pouvez fusionner un type de variable de retour et un type de variable d'erreur en un seul type de retour.

*[Documentation Result RUST](https://doc.rust-lang.org/std/result/)*

Créez une fonction `is_two()` avec le prototype suivant :

```
fn is_two(x: i32) -> Result<i32, String>
```

Elle renvoie le paramètre d'entrée si c'est un 2 sinon une erreur sous forme de `String` de votre choix.

## 2. Opérateur `?`

Avec le `Result` vient un opérateur `?` qui permet de propager les erreurs dans les `Result` aux fonctions parents (du moment que les erreurs sont du même type).

Créez une fonction `is_four()` avec le prototype suivant :

```
fn is_four(x: i32) -> Result<i32, String>
```

Utilisez un appel à la fonction `is_two()` avec l'opérateur `?` dans le corps de la fonction.

## 3. Option

Le RUST a aussi le type `Option` 