<?php

function multiplyMatrices($a, $b) {
    $rows_a = count($a);
    $cols_a = count($a[0]);
    $cols_b = count($b[0]);
    
    $result = array_fill(0, $rows_a, array_fill(0, $cols_b, 0));
    
    for ($i = 0; $i < $rows_a; $i++) {
        for ($j = 0; $j < $cols_b; $j++) {
            for ($k = 0; $k < $cols_a; $k++) {
                $result[$i][$j] += $a[$i][$k] * $b[$k][$j];
            }
        }
    }
    
    return $result;
}

function printMatrix($matrix) {
    foreach ($matrix as $row) {
        echo "[ " . implode(", ", $row) . " ]\n";
    }
}

while (true) {
    echo "\n=== Matrix Multiplication ===\n";
    echo "1. Multiply matrices\n";
    echo "2. Exit\n";
    echo "Choose: ";
    
    $choice = trim(fgets(STDIN));
    
    if ($choice == 1) {
        $matrix1 = [
            [1, 2],
            [3, 4]
        ];
        
        $matrix2 = [
            [5, 6],
            [7, 8]
        ];
        
        echo "\nMatrix 1:\n";
        printMatrix($matrix1);
        
        echo "\nMatrix 2:\n";
        printMatrix($matrix2);
        
        $result = multiplyMatrices($matrix1, $matrix2);
        
        echo "\nResult:\n";
        printMatrix($result);
        
    } else if ($choice == 2) {
        echo "Bye!\n";
        break;
    }
}

?>
