# Sujet 01 - Syntaxes et spécificités

Maintenant que tu as ton environnement de développement crée et que tu sais de quoi il est composé, tu vas pouvoir l'utiliser pour développer. 

## 0. Démarrage

Essayez de lancer `cargo run` pour compiler et lancer le binaire.

Erreur de compilation ??

Le RUST a une coding style intégrée et vérifiée à la compilation. Elle force à avoir des noms en `snake_case` pour les variables, les fonctions et les noms de binaires.

Changez le nom du binaire dans le `Cargo.toml` par le nom de votre choix du moment qu'il suit le `snake_case` :
```
[[bin]]
name = "test_bin"
path = "main.rs"
```

## 1. Variables

Créez une variable `x` qui a pour valeur `2` et affichez là. En rust pour afficher une variable utilisez `println!()`.

Exemple :
```
  println!("x is: {}", [nom_de_la_variable]);
```

[Documentation variables RUST](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

Maintenant assignez la valeur `4` à la variable `x`.

Erreur de compilation ??

## 2. Mutabilité

Les variables sont immutables de base en RUST et ne permettent donc pas la modification.

Modifiez la définition de la variable pour pouvoir changer sa valeur et réassigne la variable et ré-affiche le résultat.

## 3. Types de données complexes (String)

Le RUST étant un language de haut niveau il y a des types complexes implémentés de base dans le language comme les `String` par exemple.

Déclarez deux `String` et combine-les en une puis affiche la `String` et sa taille.

*[Documentation String RUST](https://doc.rust-lang.org/std/string/struct.String.html)*

## 4. Fonctions

Pour déclarer des fonctions la syntaxe du RUST est différentes de certains autres langages. 

Créez une fonction pour ajouter deux variables de types `i32`. Appelle la fonction avec deux nombres de ton choix et affiche son résultat.

*[Documentation fonctions RUST](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)*

## 5. Structure de contrôles

Le RUST possède quelques mots clés de structures de contrôles.

*[Documentation structures de contrôle](https://doc.rust-lang.org/book/ch03-05-control-flow.html)*

Dans votre main ajoutez une expression `if`. Utilisez cette template et modifie-là pour qu'elle compile et qu'elle affiche `Réussi` :
```
fn main() {
  let x = ...
  if x < ... {
    println!("Échec");
  } else {
    println!("Réussi");
  }
}
```
Vous pouvez aussi utiliser les structures de contrôles pour initialiser les variables. Complète cette template qui utilise le mot clé `loop` pour qu'elle affiche `Salut` :
```
fn main() {
    let mut test: String = String::new();

    let result = loop {
        ...;

        if ... == "aaaa".to_string() {
            break "Salut";
        }
    };

    println!("{}", result);
}
```

## 6. Possession et références

Cette partie est assez complexe à comprendre c'est pour cela qu'il faudra faire beaucoup de lecture. Lis la page de la [documentation RUST dessus](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

Ensuite fais compiler ce code sans modifier les lignes d'affichages (un `Vec` est un vecteur et fonctionne comme une `String` mais avec n'importe quel type) :
```
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // Ne pas changer cette ligne
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
```

## 7. Guess game

Il est temps de mettre en place tout ce que nous avons vu essaye de mettre en place un jeu de devinette de nombre avec tout ce que tu as vu depuis le début.