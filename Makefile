default:
	javac -d . src/*.java
	jar cvmf META-INF/MANIFEST.MF MissingBlobs.jar *.class
	rm *.class
