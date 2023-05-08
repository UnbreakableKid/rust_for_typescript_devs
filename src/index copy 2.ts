function practice(nums: number[], index: number): number {




    if (nums.length - 1 < index || index < 0)
        return index * 5;

    const item = nums[index];

    return item! * 5;
}


const items = [1, 2, 3]

console.log(practice(items, -1))