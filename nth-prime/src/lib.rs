pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 1; // On a déjà compté le 2
    let mut candidate = 3;

    while count <= n {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        // On passe au prochain nombre impair
        candidate += 2;
    }

    unreachable!("N'arrive jamais ici")
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // On vérifie tous les nombres de la forme 6k±1 jusqu'à la racine carrée de n
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
