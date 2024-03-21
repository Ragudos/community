ALTER TABLE IF EXISTS communities
    ADD COLUMN promotional_images TEXT[] CHECK (array_length(promotional_images, 1) <= 5);
