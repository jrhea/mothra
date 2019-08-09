SHELL := /bin/sh

include ../../config.mk

CLASS_PATH = $(ROOT_DIR)/bin/
vpath %.class $(CLASS_PATH)

NAME = mothra

examples: clean app.class

%.class:
	javac $*.java -d $(ROOT_DIR)/bin/
	#cd $(ROOT_DIR)/bin \
	#&& printf "Main-Class: app\n" > manifest.txt \
	#&& jar -cvmf manifest.txt app.jar *.class \
	#&& /Library/Java/JavaVirtualMachines/jdk1.8.0_192.jdk/Contents/Home/bin/javapackager create-image --runtime-image image --input . --output . --name mothra --main-jar app.jar

clean:
	rm -rf $(ROOT_DIR)/bin/*.class
	rm -rf $(ROOT_DIR)/bin/*.jar
