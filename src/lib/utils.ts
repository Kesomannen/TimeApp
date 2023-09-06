export function format_time(total_secs: number) {
    const hours = Math.floor(total_secs / 3600)
    const minutes = Math.floor((total_secs - (hours * 3600)) / 60)
    const secs = total_secs - (hours * 3600) - (minutes * 60)

    const hours_str = hours < 10 ? `0${hours}` : `${hours}`
    const minutes_str = minutes < 10 ? `0${minutes}` : `${minutes}`
    const secs_str = secs < 10 ? `0${secs}` : `${secs}`

    return `${hours_str}:${minutes_str}:${secs_str}`;
}

export function* map<T>(iterable: Iterable<T>, fn: (item: T) => T) {
    for (const item of iterable) {
        yield fn(item);
    }
}

export function* reduce<T>(iterable: Iterable<T>, fn: (acc: T, item: T) => T, initial: T) {
    let acc = initial;
    for (const item of iterable) {
        acc = fn(acc, item);
    }
    return acc;
}