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

## 3. Le mot clé match

Avec l'opérateur `?` l'erreur renvoyé par `is_four()` est celle de `is_two()` ce qui n'est pas très explicite pour un utilisateur alors il faudrait pouvoir renvoyer une erreur de `is_four()` à la place de celle de `is_two()`. Pour cela utiliser le mot clé `match` pour utiliser le `Result` de `is_two()` dans le fonction `is_four()`.

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

## 6. minigrep

Mettons maintenant en place un exercice pratique qui consiste à créer un grep simplifié. Vous avez acquis toutes les connaissances pour le faire.

Il y a un exemple de comment le faire dans la RUST mais il est trop complexe pour le moment. Essayez de l'implémenter avec ce que vous avez appris.