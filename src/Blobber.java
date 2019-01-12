import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Blobber {

	/*
	 * Key: Name of the blob
	 * Value: Reference to the Blob
	 */
	private HashMap<String, Blob> presentBlobs;

	/*
	 * Key: Name of blob dependency
	 * Value: Other blobs that depend on this blob
	 */
	private HashMap<String, ArrayList<Blob>> dependencyBlobs;

	/*
	 * Key: Name of missing blob dependency
	 * Value: Null (unused)
	 */
	private HashMap<String, String> missingBlobs;

	private String expandArrayList(ArrayList<Blob> arr) {
		StringBuilder expanded = new StringBuilder();

		for (Blob blob : arr) {
			expanded.append(blob.getName() + "; ");
		}

		return expanded.toString();
	}

	private ArrayList<String> listDependencies(Blob blob) {
		ArrayList<String> dependencies = new ArrayList<String>();

		/*
		 * List blob dependencies using ldd-arm:
		 * readelf -d $1 | grep "\(NEEDED\)" | sed -r "s/.*\[(.*)\]/\1/"
		 */
		try {
			ProcessBuilder pb = new ProcessBuilder();
			pb.command("bash", "-c", "readelf -d " + blob.getFile().getAbsolutePath()
					+ " | grep \"\\(NEEDED\\)\" | sed -r \"s/.*\\[(.*)\\]/\\1/\"");

			final Process p = pb.start();
			BufferedReader br = new BufferedReader(new InputStreamReader(p.getInputStream()));

			String dependency;
			while ((dependency = br.readLine()) != null) {
				dependencies.add(dependency);
			}
		} catch (Exception ex) {
			System.out.println(ex);
		}

		return dependencies;
	}

	public Blobber() {
		presentBlobs = new HashMap<String, Blob>();
		dependencyBlobs = new HashMap<String, ArrayList<Blob>>();
	}

	public void addBlobDir(String blobPath) {
		File blobDir = new File(blobPath);

		if (!blobDir.isDirectory()) {
			System.out.println("Path is not a directory!");
			return;
		}

		for (File blobFile : blobDir.listFiles()) {
			if (!blobFile.isFile())
				continue;

			Blob blob = new Blob(blobFile);
			presentBlobs.put(blob.getName(), blob);

			ArrayList<String> dependencies = listDependencies(blob);

			for (String dep : dependencies) {
				// Assumes all dependencies are *.so files
				if (!dep.endsWith(".so"))
					continue;

				ArrayList<Blob> ar;

				// Add or update dependencyBlobs
				if (!dependencyBlobs.containsKey(dep)) {
					ar = new ArrayList<Blob>();
					ar.add(blob);
					dependencyBlobs.put(dep, ar);
				} else {
					dependencyBlobs.get(dep).add(blob);
				}
			}
		}
	}

	public void updateMissingBlobs() {
		missingBlobs = new HashMap<String, String>();

		for (Map.Entry<String, ArrayList<Blob>> blob : dependencyBlobs.entrySet()) {
			String dependencyName = blob.getKey();

			if (missingBlobs.containsKey(dependencyName) || presentBlobs.containsKey(dependencyName))
				continue;

			missingBlobs.put(dependencyName, null);
		}
	}

	public void showMissingBlobs() {
		for (Map.Entry<String, String> blob : missingBlobs.entrySet()) {
			String dependencyName = blob.getKey();
			ArrayList<Blob> blobsWithDependencies = dependencyBlobs.get(dependencyName);
			System.out.println(dependencyName + " required by: " + expandArrayList(blobsWithDependencies));
		}
	}
}
