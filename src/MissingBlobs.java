public class MissingBlobs {
	public static void main(String[] args) {
		Blobber b = new Blobber();

		// Add each path to search for blobs
		for (String arg : args) {
			b.addBlobDir(arg);
		}

		b.updateMissingBlobs();
		b.showMissingBlobs();

		System.out.println();
		System.out.println("Done.");
	}
}
