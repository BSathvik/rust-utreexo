pub mod util {

    /// child gives you the left child (LSB will be 0)
    pub fn child(position: u64, forest_height: u8) -> u64 {
        let mask: u64 = (2 << forest_height) - 1;
        return (position << 1) & mask;
    }

    /// go down drop times (always left; LSBs will be 0) and return position
    pub fn child_many(position: u64, drop: u8, forest_height: u8) -> u64 {
        let mask: u64 = (2 << forest_height) - 1;
        return (position << drop) & mask;
    }

    /// Return the position of the parent of this position
    pub fn up1(position: u64, forest_height: u8) -> u64 {
        return (position >> 1) | (1 << forest_height);
    }

    /// go up rise times and return the position
    pub fn up_many(position: u64, rise: u8, forest_height: u8) -> u64 {
        let mask: u64 = (2 << forest_height) - 1;
        return (position >> rise | (mask << (forest_height - (rise - 1)))) & mask;
    }

    /// cousin returns a cousin: the child of the parent's sibling.
    /// you just xor with 2.  Actually there's no point in calling this function but
    /// it's here to document it.  If you're the left sibling it returns the left
    /// cousin.
    pub fn cousin(position: u64) -> u64 {
        return position ^ 2;
    }

    /// TODO  inForest can probably be done better a different way.
    /// do we really need this at all?  only used for error detection in descendToPos

    /// check if a node is in a forest based on number of leaves.
    /// go down and right until reaching the bottom, then check if over numleaves
    /// (same as childmany)
    pub fn in_forest(pos: u64, num_leaves: u64) -> bool {
        // quick yes:
        if pos < num_leaves {
            return true;
        }

        let h = tree_height(num_leaves);
        let marker: u64 = 1 << h;
        let mask = (marker << 1) - 1;
        if pos >= mask {
            return false;
        }
        let mut pos = pos;
        while pos & marker != 0 {
            pos = ((pos << 1) & mask) | 1
        }
        return pos < num_leaves;
    }

    /// given n leaves, how deep is the tree?
    /// iterate shifting left until greater than n
    pub fn tree_height(n: u64) -> u8 {
        let mut e: u8 = 0;
        while (1 << e) < n {
            e += 1;
        }
        return e;
    }

    /// top_pos: given a number of leaves and a height, find the position of the
    /// root at that height.  Does not return an error if there's no root at that
    /// height so watch out and check first.  Checking is easy: leaves & (1<<h)
    pub fn top_pos(leaves: u64, h: u8, forest_height: u8) -> u64 {
        let mask: u64 = (2 << forest_height) - 1;
        let before = leaves & (mask << (h + 1));
        let shifted = (before >> h) | (mask << (forest_height - (h - 1)));
        return shifted & mask;
    }

    /// getTops gives you the positions of the tree tops, given a number of leaves.
    /// LOWEST first (right to left) (blarg change this)
    pub fn get_tops_reverse(leaves: u64, forest_height: u8) -> (Vec<u64>, Vec<u8>) {
        let mut position: u64 = 0;

        // go left to right.  But append in reverse so that the tops are low to high
        // run though all bit positions.  if there's a 1, build a tree atop
        // the current position, and move to the right.
        let mut height = forest_height;
        let mut tops: Vec<u64> = vec![];
        let mut heights: Vec<u8> = vec![];

        while position < leaves {
            if (1 << height) & leaves != 0 {
                // build a tree here
                let top = up_many(position, height, forest_height);
                tops.push(top);
                heights.push(height);
                position += 1 << height
            }
            height -= 1;
        }
        return (tops, heights);
    }

    pub fn u32_b(value: u32) -> Vec<u8> {
        return value.to_be_bytes().to_vec();
    }

    pub fn b_u32(value: Vec<u8>) -> u32 {
        if value.len() != 4 {
            println!("Got {:?} to b_u32 ({} bytes)", value, value.len());
            return 0xffffffff;
        } else {
            let mut array: [u8; 4] = [0; 4];
            let value = &value;
            array.copy_from_slice(value);
            return u32::from_be_bytes(array);
        }
    }

    pub fn u64_b(value: u64) -> Vec<u8> {
        return value.to_be_bytes().to_vec();
    }

    pub fn b_u64(value: Vec<u8>) -> u64 {
        if value.len() != 8 {
            println!("Got {:?} to b_u64 ({} bytes)", value, value.len());
            return 0xffffffffffffffff;
        } else {
            let mut array: [u8; 8] = [0; 8];
            let value = &value;
            array.copy_from_slice(value);
            return u64::from_be_bytes(array);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_b_simple() {
        assert_eq!(util::u64_b(10), [0, 0, 0, 0, 0, 0, 0, 10]);
    }

    #[test]
    fn b_u64_simple() {
        assert_eq!(util::b_u64(vec![0, 0, 0, 0, 0, 0, 0, 10]), 10);
    }

    #[test]
    fn u32_b_simple() {
        assert_eq!(util::u32_b(10), [0, 0, 0, 10]);
    }

    #[test]
    fn b_u32_simple() {
        assert_eq!(util::b_u32(vec![0, 0, 0, 10]), 10);
    }
}
