import java.io.File;

public class Blob {
	private String name;
	private File file;

	public Blob(String name) {
		this.name = name;
	}

	public Blob(File file) {
		this.file = file;
		this.name = file.getName();
	}

	public File getFile() {
		return file;
	}

	public String getName() {
		return name;
	}
}
