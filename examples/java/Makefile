SHELL := /bin/sh

include ../../config.mk

CLASS_PATH = $(OUT_DIR)/
vpath %.class $(CLASS_PATH)

examples: *.class

%.class:
	javac $*.java -d $(OUT_DIR)/
	#cd $(OUT_DIR) \
	#&& printf "Main-Class: app\n" > manifest.txt \
	#&& jar -cvmf manifest.txt app.jar *.class \
	#&& /Library/Java/JavaVirtualMachines/jdk1.8.0_192.jdk/Contents/Home/bin/javapackager create-image --runtime-image image --input . --output . --name mothra --main-jar app.jar

clean:
	rm -rf $(OUT_DIR)/*.class
	rm -rf $(OUT_DIR)/*.jar
