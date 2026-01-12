package binarysearch

func SearchInts(list []int, key int) int {
	left := 0
    right := len(list) - 1
    for i, val := range list{
        mid := left + (right - left) / 2
        if(val == key){
            return i
        } else if(val > key){
            right = mid - 1
        }else{
            left = mid + 1
        }
    }

    return -1
}
