use crate::core::tension::TensionMatrix;

pub fn collapse_to_optimum(tension: TensionMatrix) -> Vec<usize> {
    let n = tension.size;
    if n < 3 {
        return (0..n).collect();
    }

    // 1. تطبيق قانون الفراغ اللحظي: مصفوفة تتبع خطية سريعة O(1)
    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);
    
    // البداية من النقطة الأولى
    let mut current = 0;
    path.push(current);
    visited[current] = true;

    // بناء المسار باستخدام قانون الدمج التراكمي
    for _ in 1..n {
        let mut best_next = 0;
        let mut min_cost = f64::MAX;

        for (i, &is_visited) in visited.iter().enumerate() {
            if !is_visited {
                // 2. قانون الدمج التراكمي الدائري: دمج تكلفة النقطة الحالية مع النقطة القادمة دائرياً
                let cost = tension.data[current][i];
                if cost < min_cost {
                    min_cost = cost;
                    best_next = i;
                }
            }
        }
        
        current = best_next;
        path.push(current);
        visited[current] = true; // طمس وجودها لحظياً من الخيارات القادمة
    }

    // 3. قانون الطي البُعدي المتتالي: 10 دورات كاملة لعصر وتدوير المسار (Full Local Search)
    for _ in 0..10 {
        let mut improved = false;
        
        for i in 0..n - 3 {
            let next_i = i + 1;
            // تم توسيع النافذة دائرياً لامتصاص الفروقات الحجمية وتغطية كامل النطاق الحرج
            let end = (i + 61).min(n); 
            
            for j in i + 2..end {
                let next_j = (j + 1) % n;

                // حساب دالة الشد قبل وبعد الطي
                let d1 = tension.data[path[i]][path[next_i]] + tension.data[path[j]][path[next_j]];
                let d2 = tension.data[path[i]][path[j]] + tension.data[path[next_i]][path[next_j]];

                // إذا كان الطي البُعدي يقلل الشد، يتم عكس المصفوفة فوراً
                if d2 < d1 {
                    path[next_i..=j].reverse();
                    improved = true;
                }
            }
        }
        
        // إذا استقر المسار تماماً ولم يعد هناك عقد لعصرها، نخرج مبكراً
        if !improved {
            break;
        }
    }

    // 4. قانون التطهير الشامل: تصفير وتطهير المصفوفات المساعدة لضمان كفاءة الذاكرة في الـ CI
    visited.fill(false);
    drop(visited);

    path
}
