# Sujet 02 - Erreurs, librairie standard et l'environnement du RUST

Maintenant que vous connaissez les fonctionnalités de bases du Rust nous allons pouvoir utiliser des fonctionnalités propres au RUST et qui facilite la vie.

## 1. Result

Pour renvoyer une erreur chaque language a sa façon de faire. En JS, throw ou renvoyé deux variables une avec le résultat l'autre avec l'erreur. En C++ pour chaque type il y a une valeur comme considérée comme valeur d'erreur. Et bien en RUST il y a le `Result`.

Result est un type templaté grace auquel vous pouvez fusionner un type de variable de retour et un type de variable d'erreur en un seul type de retour.

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

## 3. Le mot clé match

Avec l'opérateur `?` l'erreur renvoyé par `is_four()` est celle de `is_two()` ce qui n'est pas très explicite pour un utilisateur alors il faudrait pouvoir renvoyer une erreur de `is_four()` à la place de celle de `is_two()`. Pour cela utiliser le mot clé `match` pour utiliser le `Result` de `is_two()` dans la fonction `is_four()`.

*[Documentation match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)*

## 4. Option

Le RUST a aussi le type `Option` il permet de contenir soit une variable de n'importe quel type soit de contenir `None`. Cette fonctionnalité peut-être très utile car elle permet d'éviter d'avoir des valeurs par défaut sur des variables ou des champs de structures pas utilisés dans certains cas.

Faites en sorte que ce code compile :
```rust
// Vous pouvez tout modifier à l'exception de cette fonction
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(13);
    print_number(99);

    let mut numbers: [Option<u16>; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = number_to_add;
    }
}
```

## 5. Lire un fichier

Nous allons maintenant voir comment lire un fichier en Rust.

Créez un fichier sur REPL et lisez le dans votre main grâce à *[read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)*

## 6. Les tests

En plus du formatage, de la gestion de tests `cargo` propose aussi d'écrire facilement des tests (unitaires et fonctionnels).

*[Documentation sur l'écriture de tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)*

Écrivez des tests pour la fonction `is_four()` pour couvrir les cas suivants :

- Le nombre est un 4.
- Le nombre est 2.
- Le nombre n'est pas un 4 ni un 2.

## 7. minigrep

Mettons maintenant en place un exercice pratique qui consiste à créer un grep simplifié. Vous avez acquis toutes les connaissances pour le faire.

Il y a un exemple de comment le faire dans la RUST mais il est trop complexe pour le moment. Essayez de l'implémenter avec tout ce que vous avez appris.

## 8. Les macros

Si vous en êtes arrivés là pendant les 2 heures de workshop c'est que vous avec bien compris les fonctionnalités que l'on a vu et que vous arrivez bien à les utiliser. Amusons-nous un peu.

Les macros permettent d'utiliser l'AST du code et de le modifier pour ajouter des comportements spéciaux. Cela permet de réduire le code à écrire comme par exemple avec la macro `vec!` qui crée un vecteur et ajoute des éléments.

Mais les macros sont aussi très utiles car elles peuvent prendre un nombre variadique d'arguments comme avec `println!` ou `vec!`

Créez une macro (function-like) `is_four()` pour qu'elle test un nombre `x` de chiffre passés en paramètre. Exemple:

`is_four!(3, 4, 5, 1, 2)`

Si un nombre est un 4 elle l'affiche sinon elle affiche `[chiffre] is not a 4`.

*[Documentation sur les macros RUST](https://doc.rust-lang.org/book/ch19-06-macros.html)*