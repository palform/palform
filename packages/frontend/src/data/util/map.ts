export function getFromDodgyMap<K extends string | number | symbol, V>(
    map: Map<K, V> | Record<K, V>,
    key: K
): V | undefined {
    if (map instanceof Map) {
        return map.get(key);
    }
    return map[key];
}
