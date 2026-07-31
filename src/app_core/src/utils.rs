// Función para calcular la distancia de Levenshtein entre dos strings.
//TODO: Posible optimización con distancia levenshtein
pub fn levenshtein_distancia(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matriz = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matriz[i][0] = i;
    }

    for j in 0..=len2 {
        matriz[0][j] = j;
    }

    for (i, char1) in s1.chars().enumerate() {
        for (j, char2) in s2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };
            matriz[i + 1][j + 1] = (matriz[i][j + 1] + 1).min(matriz[i + 1][j] + 1).min(matriz[i][j] + cost);
        }
    }

    matriz[len1][len2]
}

pub fn order_vector(s: &str, v: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut result_dis: Vec<usize> = Vec::new();
    for val in v {
        let lev = levenshtein_distancia(&s.to_lowercase(), &val.to_lowercase());
        let mut used_index = 0;
        let mut used = false;
        for (index, value) in result_dis.iter().enumerate() {
            if lev < *value {
               result.insert(index, val.to_string());
               used_index = index;
               used = true;
               break;
            }
        }
        if used == false {
            result.push(val.to_string());
            result_dis.push(lev);
        } else {
            result_dis.insert(used_index, lev);
        }
    }
    return result;
}