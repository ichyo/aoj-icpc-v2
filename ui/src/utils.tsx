export function formatPoint(point: number) {
    if (point == 1200) {
        return "1200+";
    }
    if (point == 0) {
        return "?";
    }
    return point.toString();
};
